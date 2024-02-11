use std::collections::{HashMap, HashSet, VecDeque};
use std::future::Future;
use std::time::Duration;

use actix_web::{HttpResponse, Responder, web};
use actix_web::http::header::{ContentType};
use mobc::Pool;
use mobc_redis::redis::AsyncCommands;
use mobc_redis::RedisConnectionManager;
use rayon::prelude::*;
use redis::Commands;
use serde::{Serialize};
use serde_json::json;
use tokio::time::sleep;
use tracing::error;
use tracing::log::info;
use uuid::Uuid;

use crate::game::game_state::{GameMap, GameState, GameStatus, RoundState};
use crate::planet::direction::Direction;
use crate::planet::map_generator::MapGenerator;
use crate::planet::planet::Planet;
use crate::planet::resource::Resource;
use crate::player::{Money, PlayerState};
use crate::robot::robot::Robot;
use crate::trading::external::command::Command;
use crate::trading::external::command_type::CommandType;
use crate::trading::external::handler::battle_command_handler::{apply_damage_for_round, calculate_damage_for_round, delete_commands_for_dead_robots};
use crate::trading::external::handler::buy_command_handler::{handle_buy_commands, Item};
use crate::trading::external::handler::mining_command_handler::handle_mining_commands;
//use crate::trading::external::handler::mining_command_handler::handle_mining_command;
use crate::trading::external::handler::movement_command_handler::handle_movement_commands;
use crate::trading::external::handler::regenerate_command_handler::handle_regenerate_commands;
use crate::trading::external::handler::sell_command_handler::handle_selling_commands;

pub fn game_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_game)
        .service(delete_game)
        .service(get_all_games)
        .service(get_all_created_games)
        .service(get_game)
        .service(get_game_current_round)
        .service(delete_all_games)
        .service(get_players)
        .service(join_game)
        .service(display_map)
        .service(display_map_for_round)
        .service(display_map_for_player)
        .service(display_map_for_round_and_player)
        .service(start_game)
        .service(end_game)
        .service(handle_batch_of_commands)
        .service(get_robots_for_current_round)
        .service(get_robot_for_current_round_by_player_id_and_robot_id)
        .service(get_player_state_for_current_round)
        .service(get_player_state_for_current_round_with_xy_for_planets)
        .service(handle_batch_of_commands_hypothetically);
}

#[derive(serde::Serialize, Clone)]
struct PlanetPlayerDto {
    // View of a Planet from a players perspective
    x: usize,
    y: usize,
    movement_difficulty: u8,
    resource: Option<Resource>,
    resource_amount: u32,
    amount_of_friendly_robots: u16,
    fighting_score_friendly_robots: f32,
    amount_of_enemy_robots: u16,
    fighting_score_enemy_robots: f32,
    neighbours: HashMap<Direction, Uuid>,
}

#[derive(serde::Serialize, Clone)]
struct PlanetDto {
    x: usize,
    y: usize,
    movement_difficulty: u8,
    resource: Option<Resource>,
    resource_amount: u32,
    amount_of_friendly_robots: u16,
    fighting_score_friendly_robots: f32,
    amount_of_enemy_robots: u16,
    fighting_score_enemy_robots: f32,
    neighbours: HashMap<Direction, Uuid>,
}

#[derive(serde::Serialize, Clone)]
struct RobotDto {
    x: usize,
    y: usize,
    planet_id: Uuid,
    robot_id: Uuid,
    health: u32,
    max_health: u32,
    energy: u32,
    max_energy: u32,
    energy_regen: u32,
    storage: u32,
    max_storage: u32,
    mining_speed: u32,
    mineable_resources: Vec<Resource>,
    damage: u32,
    fighting_score: f32,
    money_value: u32,
    money_made: u32,
}

#[derive(serde::Serialize, Clone)]
struct PlayerStateDto {
    current_round: u16,
    player_name: String,
    money: u32,
    total_money_made: u32,
    map: HashMap<Uuid, PlanetPlayerDto>,
    //(x,y) -> PlanetDto
    visited_planets:  HashSet<Uuid>,
    // PlanetId -> Planet
    alive_robots: HashMap<Uuid, RobotDto>,
    alive_enemy_robots: HashMap<Uuid, RobotDto>,
    // YourRobotId -> YOurRobot
    dead_robots: HashMap<Uuid, RobotDto>,
    // YOurRobotId -> YOurRobot
    killed_robots: HashMap<Uuid, (String, RobotDto)>, // YOurRobotId -> (EnemyPlayerName, EnemyRobot)
}

#[derive(Serialize, Clone)]
pub struct RoundStateDto {
    pub round_number: u16,
    pub player_name_player_map: HashMap<String, PlayerStateDto>,
    pub map: HashMap<Uuid, PlanetPlayerDto>,
}

async fn with_game_lock<F, Fut>(redis_client: &web::Data<Pool<RedisConnectionManager>>, game_id: &String, action: F) -> Option<HttpResponse>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output=Option<HttpResponse>>,
{
    let lock_key = format!("lock:game:{}", &game_id);

    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");

    // Versuchen, den Lock zu setzen
    loop {
        let lock_set: bool = con.set_nx(&lock_key, "1").await.expect("Failed to check lock");
        if lock_set {
            // Lock erfolgreich gesetzt
            let _: bool = con.expire(&lock_key, 60).await.expect("Failed to set lock expiration");
            break;
        }

        // Wenn der Lock nicht gesetzt werden konnte, warten und erneut versuchen
        sleep(Duration::from_secs(1)).await;
    }

    // Ausführen der übergebenen Aktion
    let response = action().await;

    // Lock entfernen
    let _: () = con.del(&lock_key).await.expect("Failed to release lock");
    response
}

#[derive(serde::Deserialize)]
struct CreateGameRequestBody {
    max_rounds: u16,
    max_players: u8,
    map_size: u8,
}

#[actix_web::post("/games")]
async fn create_game(body: web::Json<CreateGameRequestBody>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = Uuid::new_v4();
    let new_game = GameState::new(
        game_id,
        body.max_rounds,
        body.max_players,
        MapGenerator::create_map(body.map_size as usize),
    );
    //save game to redis
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let _: () = con.set(format!("games/{}", game_id.to_string()), serde_json::to_string(&new_game).unwrap()).await.expect("Failed to set key");
    return HttpResponse::Created().insert_header(ContentType::json()).body(json!({
        "game_id": game_id
    }).to_string());
}

#[actix_web::delete("/games/{game_id}")]
async fn delete_game(path: web::Path<String>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = path.into_inner();
    with_game_lock(&redis_client, &game_id, || async {
        {
            let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
            let _: () = con.del(format!("games/{}", &game_id)).await.expect("Failed to delete key");
            return Some(HttpResponse::Ok().insert_header(ContentType::json()).body(json!({
                "game_id": game_id
            }).to_string()));
        }
    }).await.unwrap_or_else(|| HttpResponse::NotFound().body("Game not found"))
}

#[actix_web::get("/games")]
async fn get_all_games(redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let mut con = redis_client.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {}", e);
        HttpResponse::InternalServerError().finish()
    }).expect("Failed to get Redis connection from pool");
    let mut con2 = redis_client.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {}", e);
        HttpResponse::InternalServerError().finish()
    }).expect("Failed to get Redis connection from pool");

    let mut game_ids = match con.scan_match::<String, String>("games/*".to_string()).await {
        Ok(ids) => ids,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let mut games_states: Vec<GameState> = Vec::new();
    while let Some(key) = game_ids.next_item().await {
        let game: String = con2.get(&key).await.expect("Failed to get game");
        let game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
        games_states.push(game_state);
    }
    if games_states.is_empty() {
        return HttpResponse::NotFound().body("No games found");
    }
    HttpResponse::Ok().json(games_states)
}

#[actix_web::get("/games/created")]
async fn get_all_created_games(redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let mut con = redis_client.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {}", e);
        HttpResponse::InternalServerError().finish()
    }).expect("Failed to get Redis connection from pool");
    let mut con2 = redis_client.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {}", e);
        HttpResponse::InternalServerError().finish()
    }).expect("Failed to get Redis connection from pool");

    let mut game_ids = match con.scan_match::<String, String>("games/*".to_string()).await {
        Ok(ids) => ids,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let mut created_games_states: Vec<GameState> = Vec::new();
    while let Some(key) = game_ids.next_item().await {
        let game: String = con2.get(&key).await.expect("Failed to get game");
        let mut game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
        if game_state.status == GameStatus::Created {
            game_state.round_states.clear(); // Not relevant for this route.
            created_games_states.push(game_state);
        }
    }
    if created_games_states.is_empty() {
        return HttpResponse::NotFound().body("No games found");
    }
    HttpResponse::Ok().json(created_games_states)
}

#[actix_web::get("/games/{game_id}")]
async fn get_game(path: web::Path<String>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = path.into_inner();
    let mut con = redis_client.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {}", e);
        HttpResponse::InternalServerError().finish()
    }).expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
    let game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
    HttpResponse::Ok().json(game_state)
}

#[actix_web::get("/games/{game_id}/currentRound")]
async fn get_game_current_round(path: web::Path<String>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = path.into_inner();
    let mut con = redis_client.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {}", e);
        HttpResponse::InternalServerError().finish()
    }).expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
    let game_state: &mut GameState = &mut serde_json::from_str(game.as_str()).unwrap();
    let current_round = game_state.current_round;
    let round_state = game_state.round_states.remove(&current_round).unwrap();
    game_state.round_states.clear();
    game_state.round_states.insert(current_round, round_state);
    HttpResponse::Ok().json(game_state)
}

#[actix_web::get("/games/{game_id}/currentRound/new")]
async fn get_game_current_round_new(path: web::Path<String>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = path.into_inner();
    let mut con = redis_client.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {}", e);
        HttpResponse::InternalServerError().finish()
    }).expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
    let game_state: &mut GameState = &mut serde_json::from_str(game.as_str()).unwrap();
    let current_round: u16 = game_state.current_round;
    let current_round_state: RoundState = game_state.round_states.remove(&current_round).unwrap();

    //let PlayerStateDto


    game_state.round_states.clear();
    game_state.round_states.insert(current_round, current_round_state.clone());
    HttpResponse::Ok().json(game_state)
}


#[actix_web::delete("/games")]
async fn delete_all_games(redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let mut con = redis_client.get().await.map_err(|e| {
        error!("Failed to get Redis connection: {}", e);
        HttpResponse::InternalServerError().finish()
    }).expect("Failed to get Redis connection from pool");

    let mut game_keys = match con.scan_match::<String, String>("games/*".to_string()).await {
        Ok(ids) => ids,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let mut deleted_games = Vec::new();
    while let Some(key) = game_keys.next_item().await {
        // Assuming with_game_lock is an async function
        // Ensure that this function handles errors and lock logic correctly
        with_game_lock(&redis_client, &key, || async {
            let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
            let _: () = con.del(&key).await.expect("Failed to delete key");
            None
        }).await;
        deleted_games.push(key);
    }
    if deleted_games.is_empty() {
        return HttpResponse::NotFound().body("No games found");
    }
    HttpResponse::Ok().json(deleted_games)
}

#[actix_web::get("/games/{game_id}/players")]
async fn get_players(path: web::Path<String>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = path.into_inner();
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
    let game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
    return HttpResponse::Ok().insert_header(ContentType::json()).body(json!({
        "participating_players": game_state.participating_players
    }).to_string());
}

#[derive(serde::Deserialize)]
pub struct JoinGameRequestBody {
    player_name: String,
}

#[actix_web::put("/games/{game_id}")]
async fn join_game(body: web::Json<JoinGameRequestBody>, path: web::Path<String>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = path.into_inner();
    let starting_money: u32 = 500;
    let player = PlayerState {
        player_name: body.player_name.to_string(),
        money: Money { amount: starting_money },
        total_money_made: Money { amount: starting_money },
        visited_planets: HashSet::new(),
        commands: vec![
            (CommandType::SELLING, VecDeque::new()),
            (CommandType::BUYING, VecDeque::new()),
            (CommandType::MOVEMENT, VecDeque::new()),
            (CommandType::BATTLE, VecDeque::new()),
            (CommandType::MINING, VecDeque::new()),
            (CommandType::REGENERATE, VecDeque::new()),
        ].into_iter().collect(),
        robots: HashMap::new(),
        killed_robots: HashMap::new(),
    };
    with_game_lock(&redis_client, &game_id, || async {
        {
            let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
            let game: String = con.get(format!("games/{}", &game_id)).await.unwrap_or(None)?;
            let mut game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
            if game_state.status != GameStatus::Created {
                return Some(HttpResponse::BadRequest().body(format!("Game {} can't be joined because it is currently in status {:?}", &game_id, &game_state.status)));
            }
            if game_state.participating_players.contains(&body.player_name) {
                return Some(HttpResponse::BadRequest().body(format!("Game {} can't be joined because player {} has already joined", &game_id, &body.player_name)));
            }
            game_state.participating_players.push(player.player_name.clone());
            let round_state = game_state.round_states.get_mut(&0).unwrap();
            round_state.player_name_player_map.insert(body.player_name.to_string(), player);
            let is_write_successful: bool = con.set(format!("games/{}", &game_id), serde_json::to_string(&game_state).unwrap()).await.unwrap_or(false);
            if !is_write_successful {
                return Some(HttpResponse::InternalServerError().body(format!("Failed to write game {} to Redis", &game_id)));
            }
            return Some(HttpResponse::Ok().insert_header(ContentType::json()).body(json!({
                "player_name" : body.player_name,
                "game_id": game_id,
                "money": starting_money,
            }).to_string()));
        }
    }).await.unwrap_or(HttpResponse::NotFound().body(format!("Game {game_id} can't be joined because it was not found.")))
}


#[actix_web::get("/games/{game_id}/map")]
async fn display_map(path: web::Path<String>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = path.into_inner();
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
    let game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
    let current_round_status = &game_state.round_states.par_iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().1;
    let planets = &current_round_status.map.planets;
    let planets_as_ref = &planets.par_iter().map(|row| {
        row.par_iter().map(|planet_option| planet_option.as_ref()).collect()
    }).collect();
    let game_map_as_string = MapGenerator::display_map_with_connections(planets_as_ref);
    return HttpResponse::Ok().body(game_map_as_string);
}

#[actix_web::get("/games/{game_id}/map/rounds/{round_number}")]
async fn display_map_for_round(path: web::Path<(String, u16)>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let (game_id, round_number) = path.into_inner();
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
    let game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
    let planets = &game_state.round_states[&round_number].map.planets;
    let planets_as_ref: &Vec<Vec<Option<&Planet>>> = &planets.par_iter().map(|row| {
        row.par_iter().map(|planet_option| planet_option.as_ref()).collect()
    }).collect();
    let game_map_as_string = MapGenerator::display_map_with_connections(planets_as_ref);
    return HttpResponse::Ok().body(game_map_as_string);
}

#[actix_web::get("/games/{game_id}/map/players/{player_name}")]
async fn display_map_for_player(path: web::Path<(String, String)>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let (game_id, player_name) = path.into_inner();
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
    let game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
    let latest_round_state = game_state.round_states.par_iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().1;
    let player_state = &latest_round_state.player_name_player_map.get(&player_name).unwrap();
    let planets: Vec<Vec<Option<&Planet>>> = latest_round_state.map.planets.par_iter().map(|row| {
        row.par_iter().map(|planet_option| {
            planet_option.as_ref().and_then(|planet| {
                if player_state.visited_planets.contains(&planet.planet_id) {
                    Some(planet)
                } else {
                    None
                }
            })
        }).collect()
    }).collect();

    let game_map_as_string = MapGenerator::display_map_with_connections(&planets);
    return HttpResponse::Ok().body(game_map_as_string);
}


#[actix_web::get("/games/{game_id}/map/rounds/{round_number}/players/{player_name}")]
async fn display_map_for_round_and_player(path: web::Path<(String, u16, String)>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let (game_id, round_number, player_name) = path.into_inner();
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
    let game_state: GameState = serde_json::from_str(game.as_str()).unwrap();

    if !game_state.round_states.contains_key(&round_number) {
        return HttpResponse::NotFound().body(format!("Round {} not found", round_number));
    }
    let planet_state_for_round = &game_state.round_states[&round_number].map.planets;
    if !game_state.round_states[&round_number].player_name_player_map.contains_key(&player_name) {
        return HttpResponse::NotFound().body(format!("Player {} not found", player_name));
    }

    let known_planets_to_player = &game_state.round_states[&round_number].player_name_player_map[&player_name].visited_planets;
    let planets: Vec<Vec<Option<&Planet>>> = planet_state_for_round.par_iter().map(|row| {
        row.par_iter().map(|planet_option| {
            planet_option.as_ref().and_then(|planet| {
                if known_planets_to_player.contains(&planet.planet_id) {
                    Some(planet)
                } else {
                    None
                }
            })
        }).collect()
    }).collect();

    let game_map_as_string = MapGenerator::display_map_with_connections(&planets);
    return HttpResponse::Ok().body(game_map_as_string);
}

#[actix_web::post("/games/{game_id}/gameCommands/start")]
async fn start_game(path: web::Path<String>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = path.into_inner();
    with_game_lock(&redis_client, &game_id, || async {
        {
            let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
            let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
            let mut game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
            if game_state.status != GameStatus::Created {
                return Some(HttpResponse::BadRequest().body(format!("Game {} can't be started because it is currently in status {:?}", &game_id, &game_state.status)));
            }
            if game_state.participating_players.len() == 0 {
                return Some(HttpResponse::BadRequest().body(format!("Game {} can't be started because no player has joined yet", &game_id)));
            }
            game_state.status = GameStatus::Started;
            let is_write_successful: bool = con.set(format!("games/{}", &game_id), serde_json::to_string(&game_state).unwrap()).await.unwrap_or(false);
            if !is_write_successful {
                return Some(HttpResponse::InternalServerError().body(format!("Failed to write game {} to Redis", &game_id)));
            }
            return Some(HttpResponse::Ok().insert_header(ContentType::json()).body(json!({
                "game_id": game_id,
                "game_status": game_state.status,
            }).to_string()));
        }
    }).await.unwrap_or(HttpResponse::NotFound().body(format!("Game {game_id} can't be started because it was not found.")))
}

#[actix_web::post("/games/{game_id}/gameCommands/end")]
async fn end_game(path: web::Path<String>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = path.into_inner();
    with_game_lock(&redis_client, &game_id, || async {
        {
            let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
            let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
            let mut game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
            if game_state.status != GameStatus::Started {
                return Some(HttpResponse::BadRequest().body(format!("Game {} can't be ended because it is currently in status {:?}", &game_id, &game_state.status)));
            }
            game_state.status = GameStatus::Ended;
            let is_write_successful: bool = con.set(format!("games/{}", &game_id), serde_json::to_string(&game_state).unwrap()).await.unwrap_or(false);
            if !is_write_successful {
                return Some(HttpResponse::InternalServerError().body(format!("Failed to write game {} to Redis", &game_id)));
            }
            return Some(HttpResponse::Ok().insert_header(ContentType::json()).body(json!({
                "game_id": game_id,
                "game_status": game_state.status,
            }).to_string()));
        }
    }).await.unwrap_or(HttpResponse::NotFound().body(format!("Game {game_id} can't be ended because it was not found.")))
}

fn all_players_submitted_commands(game_state: &GameState) -> bool {
    let current_round = game_state.current_round;
    let round_state = game_state.round_states.get(&current_round).unwrap();
    let players = &round_state.player_name_player_map;
    players.values().all(|player| {
        let player_has_commands = player.commands.values().any(|commands| !commands.is_empty());
        let player_without_robots_has_buying_command = player.robots.is_empty() && player.commands.values().any(|commands| commands.iter().any(|command| command.command_type == CommandType::BUYING));

        let mut robot_ids_for_player = player.robots.keys().clone().collect::<HashSet<&Uuid>>();
        let robot_amount = robot_ids_for_player.len();

        //When a player has no robots, he needs to submit atleast one Buying Robot Command.
        //If he has robots, he needs to submit atleast one command for each alive robot that he owns.
        //Commands for robots can be Buying (CommandType::Buying, but has a robot_id in the commandObject), Selling, Movement, Battle, Mining, Regenerate, so you have to check for all of them.
        for (_, command_queue) in player.commands.iter() {
            for command in command_queue.iter() {
                if let Some(robot_id) = command.command_object.robot_id {
                    //Remove robot_id from robot_ids_for_player
                    robot_ids_for_player.remove(&robot_id);
                }
            }
        }
        let player_with_robots_has_commands_for_every_robot = robot_ids_for_player.is_empty() && robot_amount > 0;

        info!("Player {} has commands: {}, player_without_robots_has_buying_command: {}, player_with_robots_has_commands_for_every_robot: {} Number left in set: {}", player.player_name, player_has_commands, player_without_robots_has_buying_command, player_with_robots_has_commands_for_every_robot, robot_ids_for_player.len());

        player_has_commands && (player_without_robots_has_buying_command || player_with_robots_has_commands_for_every_robot)
    })
}

async fn process_commands_for_round(mut game_state: GameState) -> Option<GameState> {
    let current_round = game_state.current_round;
    let old_round_state = game_state.round_states.get(&current_round).unwrap().clone();

    handle_selling_commands(&mut game_state);
    handle_buy_commands(&mut game_state);
    handle_movement_commands(&mut game_state);
    //Battle
    let damage_reports = calculate_damage_for_round(&mut game_state).await;
    apply_damage_for_round(damage_reports, &mut game_state);
    delete_commands_for_dead_robots(&mut game_state);
    handle_mining_commands(&mut game_state);
    handle_regenerate_commands(&mut game_state);

    let mut new_round_state = game_state.round_states.get_mut(&current_round).unwrap().clone();
    //TODO: Potential flaw in Roundnumber Logic error handling if games are supposed to end.
    game_state.round_states.insert(current_round, old_round_state);
    game_state.start_next_round();
    new_round_state.round_number = game_state.current_round;
    game_state.round_states.insert(game_state.current_round, new_round_state);
    Some(game_state)
}

#[actix_web::post("/games/{game_id}/commands")]
async fn handle_batch_of_commands(mut body: web::Json<Vec<Command>>, path: web::Path<String>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    /*
    Commands are executed in the following order:
    1. Trading
    2. Moving
    3. Repairing (Buying a health or energy restore)
    4. Battleing (only possible when on same planet)
    5. Mining
    6. Regenerating
     */
    if body.0.is_empty() {
        return HttpResponse::BadRequest().body("No commands found");
    }

    let game_id = path.into_inner();
    let player_name = body.get(0).unwrap().player_name.clone();
    let commands = body.0;

    with_game_lock(&redis_client, &game_id, || async {
        let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
        let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
        let mut game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
        if game_state.status != GameStatus::Started {
            return Some(HttpResponse::BadRequest().body(format!("Game {} can't take commands because it is currently in status {:?}", &game_id, &game_state.status)));
        }
        let current_round = game_state.current_round;
        let round_state = game_state.round_states.get_mut(&current_round).unwrap();
        let player = round_state.player_name_player_map.get_mut(&player_name).unwrap();
        if player.commands.values().any(|commands| !commands.is_empty()) {
            error!("Overwriting commands for player {}, before : {:?} ", player.player_name, player.commands);
            player.commands.clear();
        }
        for command in commands {
            player.commands.entry(command.command_type)
                .or_insert_with(VecDeque::new)
                .push_back(command);
        }
        info!("Player {} submitted commands: {:?}", player.player_name, player.commands);

        if all_players_submitted_commands(&game_state) {
            let mut game_state = process_commands_for_round(game_state).await.unwrap();
            // Zähle die Anzahl der Spieler, die sich keine Roboter leisten können
            let mut cannot_afford_robot_count = 0;
            let player_count = game_state.round_states.get(&game_state.current_round).unwrap().player_name_player_map.len();

            for player in game_state.round_states.get(&game_state.current_round).unwrap().player_name_player_map.values() {
                // Wenn der Spieler keine ALIVE Robots mehr hat und kein Geld sich neue zu kaufen, dann kann er sich keine Roboter leisten
                let no_robots_alive = player.robots.iter().all(|robot| !robot.1.is_alive());
                if no_robots_alive && player.money.amount < Item::Robot(1).get_cost() {
                    cannot_afford_robot_count += 1;
                    info!("Player {} has no robots and can't afford to buy a new robot. HE LOST!", player.player_name);
                }
            }

            // Setze current_round auf max_rounds, wenn alle bis auf maximal einen Spieler sich keine Roboter leisten können
            if cannot_afford_robot_count >= player_count - 1 && player_count > 1 {
                game_state.current_round = game_state.max_rounds;
            }
            let is_write_successful: bool = con.set(format!("games/{}", &game_id), serde_json::to_string(&game_state).unwrap()).await.unwrap_or(false);
            if !is_write_successful {
                return Some(HttpResponse::InternalServerError().body(format!("Failed to save game {} to Redis", &game_id)));
            }
            return Some(HttpResponse::Ok().finish());
        }

        let is_write_successful: bool = con.set(format!("games/{}", &game_id), serde_json::to_string(&game_state).unwrap()).await.unwrap_or(false);
        if !is_write_successful {
            return Some(HttpResponse::InternalServerError().body(format!("Failed to save game {} to Redis", &game_id)));
        }
        return Some(HttpResponse::Accepted().body("Waiting for other players to submit commands"));
    })
        .await
        .unwrap_or(HttpResponse::NotFound().body(format!("Game {game_id} can't take commands because it was not found.")))
}

#[actix_web::get("/games/{game_id}/currentRound/players/{player_name}/robots")]
async fn get_robots_for_current_round(path: web::Path<(String, String)>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let (game_id, player_name) = path.into_inner();
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
    let mut game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
    let robots = game_state.get_robots_for_current_round(&player_name).unwrap();
    HttpResponse::Ok().json(robots)
}

#[actix_web::get("/games/{game_id}/currentRound/players/{player_name}/robots/{robot_id}")]
async fn get_robot_for_current_round_by_player_id_and_robot_id(path: web::Path<(String, String, String)>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let (game_id, player_name, robot_id) = path.into_inner();
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
    let mut game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
    let robot = game_state.get_robot_for_current_round_by_player_id_and_robot_id(&player_name, &Uuid::parse_str(&robot_id).unwrap()).unwrap();
    HttpResponse::Ok().json(robot)
}


#[actix_web::get("/games/{game_id}/currentRound/players/{player_name}")]
async fn get_player_state_for_current_round(path: web::Path<(String, String)>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let (game_id, player_name) = path.into_inner();
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(&format!("Failed to get game {} . Maybe a non existent gameid was passed.", &game_id));
    let game_state: GameState = serde_json::from_str(&game).unwrap();
    let player_state = game_state.get_player_for_current_round(&player_name).unwrap();
    let enemy_robots = game_state.get_enemy_robots_for_current_round(&player_name).unwrap_or_else(|| Vec::new());

    #[derive(serde::Serialize)]
    struct PlayerStateDto<'a> {
        current_round: u16,
        player_name: String,
        money: u32,
        total_money_made: u32,
        visited_planets: HashMap<Uuid, &'a Planet>,
        // PlanetId -> Planet
        alive_robots: HashMap<Uuid, &'a Robot>,
        alive_enemy_robots: Vec<&'a Robot>,
        // YourRobotId -> YOurRobot
        dead_robots: HashMap<Uuid, &'a Robot>,
        // YOurRobotId -> YOurRobot
        killed_robots: &'a HashMap<Uuid, (String, Robot)>, // YOurRobotId -> (EnemyPlayerName, EnemyRobot)
    }

    let player_state_dto = PlayerStateDto {
        current_round: game_state.current_round,
        player_name: player_state.player_name.clone(),
        money: player_state.money.amount,
        total_money_made: player_state.total_money_made.amount,
        visited_planets: player_state.visited_planets.iter().map(|(&planet_id)| {
            let (x, y) = game_state.round_states[&game_state.current_round].map.indices.get(&planet_id).expect("Planet not found in indices");
            let planet = game_state.round_states[&game_state.current_round].map.planets[*x][*y].as_ref().unwrap();
            (planet_id, planet)
        }).collect(),
        alive_robots: player_state.robots.iter().filter(|(_, robot)| robot.is_alive()).map(|(&robot_id, robot)| (robot_id, robot)).collect(),
        alive_enemy_robots: enemy_robots.iter().filter(|robot| robot.is_alive()).map(|robot| *robot).collect(),
        dead_robots: player_state.robots.iter().filter(|(_, robot)| !robot.is_alive()).map(|(&robot_id, robot)| (robot_id, robot)).collect(),
        killed_robots: &player_state.killed_robots,
    };

    HttpResponse::Ok().json(player_state_dto)
}

fn get_player_state_dto_from_gamestate(game_state: GameState, player_name: &str) -> PlayerStateDto {
    let map = &game_state.round_states[&game_state.current_round].map;

    let player_state = game_state.get_player_for_current_round(&player_name).unwrap();
    // Function to create RobotDto HashMap
    let (alive_robots, dead_robots) = player_state.robots.iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut alive, mut dead), (robot_id, robot)| {
            let (x, y) = map.indices.get(&robot.planet_id).expect("Planet not found in indices");
            let robot_dto = RobotDto {
                x: *x,
                y: *y,
                robot_id: *robot_id,
                planet_id: robot.planet_id,
                health: robot.health,
                max_health: robot.levels.get_health_for_level(),
                energy: robot.energy,
                max_energy: robot.levels.get_energy_for_level(),
                energy_regen: robot.levels.get_energy_regen_for_level(),
                storage: robot.get_used_storage_space(),
                max_storage: robot.levels.get_storage_for_level(),
                mining_speed: robot.levels.get_mining_speed_for_level(),
                mineable_resources: robot.get_mineable_resources(),
                damage: robot.levels.get_damage_for_level(),
                fighting_score: robot.get_fighting_score(),
                money_value: robot.get_money_costs_for_robots_existing_upgrades(),
                money_made: robot.money_made
            };
            if robot.health > 0 {
                alive.insert(*robot_id, robot_dto);
            } else {
                dead.insert(*robot_id, robot_dto);
            }
            (alive, dead)
        },
    );

    let (alive_enemy_robots, dead_enemy_robots) = game_state.get_enemy_robots_for_current_round(&player_name).unwrap_or_else(|| Vec::new()).iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut alive, mut dead), robot| {
            let (x, y) = map.indices.get(&robot.planet_id).expect("Planet not found in indices");
            let robot_dto = RobotDto {
                x: *x,
                y: *y,
                robot_id: robot.robot_id,
                planet_id: robot.planet_id,
                health: robot.health,
                max_health: robot.levels.get_health_for_level(),
                energy: robot.energy,
                max_energy: robot.levels.get_energy_for_level(),
                energy_regen: robot.levels.get_energy_regen_for_level(),
                storage: robot.get_used_storage_space(),
                max_storage: robot.levels.get_storage_for_level(),
                mining_speed: robot.levels.get_mining_speed_for_level(),
                mineable_resources: robot.get_mineable_resources(),
                damage: robot.levels.get_damage_for_level(),
                fighting_score: robot.get_fighting_score(),
                money_value: robot.get_money_costs_for_robots_existing_upgrades(),
                money_made: robot.money_made
            };
            if robot.health > 0 {
                alive.insert(robot.robot_id, robot_dto);
            } else {
                dead.insert(robot.robot_id, robot_dto);
            }
            (alive, dead)
        },
    );
    // Compute planet data in parallel
    let planet_map: HashMap<Uuid, PlanetPlayerDto> = map.indices.values().par_bridge().map(|&(x, y)| {
        let planet = map.planets[x][y].as_ref().unwrap();
        let resource_data = planet.resources.as_ref().map(|(r, a)| (Some(r.clone()), *a)).unwrap_or((None, 0));

        let friendly_count_and_score = alive_robots.values().filter(|robot| robot.x == x && robot.y == y)
            .fold((0, 0.0), |(count, score), robot| (count + 1, score + robot.fighting_score));
        let enemy_count_and_score = alive_enemy_robots.values().filter(|robot| robot.x == x && robot.y == y)
            .fold((0, 0.0), |(count, score), robot| (count + 1, score + robot.fighting_score));

        let planet_dto = PlanetPlayerDto {
            x,
            y,
            movement_difficulty: planet.movement_difficulty,
            resource: resource_data.0,
            resource_amount: resource_data.1,
            amount_of_friendly_robots: friendly_count_and_score.0 as u16,
            fighting_score_friendly_robots: friendly_count_and_score.1,
            amount_of_enemy_robots: enemy_count_and_score.0 as u16,
            fighting_score_enemy_robots: enemy_count_and_score.1,
            neighbours: planet.neighbours.clone(),
        };
        (planet.planet_id, planet_dto)
    }).collect();

    PlayerStateDto {
        current_round: game_state.current_round,
        player_name: player_state.player_name.clone(),
        money: player_state.money.amount,
        total_money_made: player_state.total_money_made.amount,
        map: planet_map,
        visited_planets: player_state.clone().visited_planets,
        alive_robots: alive_robots,
        alive_enemy_robots: alive_enemy_robots,
        dead_robots: dead_robots,
        killed_robots: player_state.killed_robots.iter().map(|(robot_id, (enemy_player_name, robot))| {
            let (x, y) = map.indices.get(&robot.planet_id).expect("Planet not found in indices");
            let robot_dto = RobotDto {
                x: *x,
                y: *y,
                robot_id: *robot_id,
                planet_id: robot.planet_id,
                health: robot.health,
                max_health: robot.levels.get_health_for_level(),
                energy: robot.energy,
                max_energy: robot.levels.get_energy_for_level(),
                energy_regen: robot.levels.get_energy_regen_for_level(),
                storage: robot.get_free_storage_space(),
                max_storage: robot.levels.get_storage_for_level(),
                mining_speed: robot.levels.get_mining_speed_for_level(),
                mineable_resources: robot.get_mineable_resources(),
                damage: robot.stats.damage,
                fighting_score: robot.get_fighting_score(),
                money_value: robot.get_money_costs_for_robots_existing_upgrades(),
                money_made: robot.money_made
            };
            (*robot_id, (enemy_player_name.clone(), robot_dto))
        }).collect::<HashMap<Uuid,(String,RobotDto)>>().clone(),
    }
}

#[actix_web::get("/games/{game_id}/currentRound/players/{player_name}/new")]
async fn get_player_state_for_current_round_with_xy_for_planets(path: web::Path<(String, String)>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let (game_id, player_name) = path.into_inner();
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(&format!("Failed to get game {} . Maybe a non existent gameid was passed.", &game_id));
    let game_state: GameState = serde_json::from_str(&game).unwrap();
    let player_state_dto = get_player_state_dto_from_gamestate(game_state, &player_name);
    HttpResponse::Ok().json(player_state_dto)
}


#[actix_web::post("/games/{game_id}/commands/hypothetically")]
async fn handle_batch_of_commands_hypothetically(mut body: web::Json<Vec<Command>>, path: web::Path<String>, redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    /*
    Commands are executed in the following order:
    1. Trading
    2. Moving
    3. Repairing (Buying a health or energy restore)
    4. Battleing (only possible when on same planet)
    5. Mining
    6. Regenerating
     */
    if body.0.is_empty() {
        return HttpResponse::BadRequest().body("No commands found");
    }

    let game_id = path.into_inner();
    let player_name = body.get(0).unwrap().player_name.clone();
    let commands = body.0;


    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let game: String = con.get(format!("games/{}", &game_id)).await.expect(format!("Failed to get game {}", game_id).as_str());
    let mut game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
    if game_state.status != GameStatus::Started {
        return HttpResponse::BadRequest().body(format!("Game {} can't take commands because it is currently in status {:?}", &game_id, &game_state.status));
    }
    let current_round = game_state.current_round;
    let round_state = game_state.round_states.get_mut(&current_round).unwrap();
    let player = round_state.player_name_player_map.get_mut(&player_name).unwrap();
    if player.commands.values().any(|commands| !commands.is_empty()) {
        error!("Overwriting commands for player {}, before : {:?} ", player.player_name, player.commands);
        player.commands.clear();
    }
    for command in commands {
        player.commands.entry(command.command_type)
            .or_insert_with(VecDeque::new)
            .push_back(command);
    }
    info!("Player {} submitted commands: {:?}", player.player_name, player.commands);
    let game_state = process_commands_for_round(game_state).await.unwrap();
    let player_state_dto = get_player_state_dto_from_gamestate(game_state, &player_name);
    return HttpResponse::Ok().json(player_state_dto);
}