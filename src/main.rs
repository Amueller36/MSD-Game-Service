use actix_cors::Cors;
use actix_web::{HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use mobc_redis::RedisConnectionManager;
use redis::Commands;
use tracing::{debug, error, info, Level};
use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::prelude::*;

mod game;
mod planet;

mod robot;

mod trading;
mod api;
mod player;
#[actix_web::get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse:: Ok().body("Hello, world!")
}
#[actix_web::main]
async fn main() -> Result<(),std::io::Error>{
    let redis_host = std::env::var("REDIS_HOST").unwrap_or("127.0.0.1".into());
    let redis_port : String = std::env::var("REDIS_PORT").unwrap_or("6379".into());
    let file_appender = tracing_appender::rolling::daily("./logs", "MSD.log");
    //let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout()); //für console logging
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(EnvFilter::new("debug"))
        .with_ansi(true)
        .init();
    debug!("Starting server");
    let client = mobc_redis::redis::Client::open(format!("redis://{}:{}",redis_host,redis_port)).expect("Invalid redis url");
    let pool_manager = RedisConnectionManager::new(client);
    let pool = mobc::Pool::builder()
        .max_open(50)
        .max_idle(10)
        .build(pool_manager);
    let pool_as_sharable_data = Data::new(pool);
    HttpServer::new(move || {
        actix_web::App::new()
            .wrap(
                Cors::permissive()
            )
            .app_data(Data::clone(&pool_as_sharable_data))
            .service(hello_world)
            .configure(api::games::game_routes)
    })
        .bind("0.0.0.0:8080")
        .expect("Failed to bind to port")
        .run()
        .await
}
