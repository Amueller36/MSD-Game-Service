use actix_web::{HttpResponse, Responder, web};
use mobc::Pool;
use mobc_redis::redis::AsyncCommands;
use mobc_redis::RedisConnectionManager;
use redis::{Commands, Connection};
use tokio::sync::Mutex;
use tracing::info;
use tracing::log::debug;
use uuid::Uuid;
use crate::game;
use crate::game::game_state::GameState;
use crate::planet::map_generator::MapGenerator;

#[actix_web::post("/games")]
async fn create_game(mut redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = Uuid::new_v4();
    let new_game = GameState::new(game_id, MapGenerator::create_map(10));
    //save game to redis
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let _: () = con.set(format!("games/{}",game_id.to_string()), serde_json::to_string(&new_game).unwrap()).await.expect("Failed to set key");
    return HttpResponse::Ok().body(serde_json::to_string(&new_game.game_id).unwrap());
}

#[actix_web::delete("/games/{game_id}")]
async fn delete_games( path: web::Path<String>,redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = path.into_inner();
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let _: () = con.del(&game_id).await.expect("Failed to delete all games");
    return HttpResponse::Ok().body(format!("Deleted game {}", game_id));
}
#[actix_web::get("/games/{game_id}")]
async fn join_game(redis_client: web::Data<redis::Connection>) -> impl Responder {
    return HttpResponse::Ok().body("Hello, world!");

    todo!("Create a PlayerId and add it to the game and return the PlayerId");
}

#[actix_web::get("/games/{game_id}/map")]
async fn display_map(path : web::Path<String>,redis_client: web::Data<Pool<RedisConnectionManager>>) -> impl Responder {
    let game_id = path.into_inner();
    let mut con = redis_client.get().await.expect("Failed to get Redis connection from pool");
    let game: String = con.get(&game_id).await.expect(format!("Failed to get game {}",game_id).as_str());
    let game_state: GameState = serde_json::from_str(game.as_str()).unwrap();
    return HttpResponse::Ok().body("Hello, world!");

    todo!("Return the map of the game");
}
#[actix_web::get("/games/{game_id}/map/rounds/{round_number}")]
async fn display_map_for_round(redis_client: web::Data<redis::Connection>) -> impl Responder {
    return HttpResponse::Ok().body("Hello, world!");

    todo!("Return the map of the game");
}

#[actix_web::post("/games/{game_id}/gameCommands/start")]
async fn start_game(redis_client: web::Data<redis::Connection>) -> impl Responder {
    return HttpResponse::Ok().body("Hello, world!");

    todo!("Start the game and return the RoundState");
}

#[actix_web::post("/games/{game_id}/gameCommands/end")]
async fn end_game(redis_client: web::Data<redis::Connection>) -> impl Responder {
    return HttpResponse::Ok().body("Hello, world!");

    todo!("End the game and return the GameState");
}
