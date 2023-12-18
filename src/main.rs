use std::clone;
use std::sync::Arc;
use actix_web::{HttpResponse, HttpServer, Responder, web};
use actix_web::web::{Data, service, to};
use mobc_redis::RedisConnectionManager;
use redis::Commands;
use tokio::sync::Mutex;

mod game;
mod planet;

mod robot;

mod trading;
mod api;
#[actix_web::get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse:: Ok().body("Hello, world!")
}
#[actix_web::main]
async fn main() -> Result<(),std::io::Error>{
    tracing_subscriber::fmt::init();
    let client = mobc_redis::redis::Client::open("redis://0.0.0.0:6379").expect("Invalid redis url");
    let pool_manager = RedisConnectionManager::new(client);
    let pool = mobc::Pool::builder().build(pool_manager);
    let pool_data = Data::new(pool);
    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(Data::clone(&pool_data))
            .service(hello_world)
            .service(api::games::create_game)
            .service(api::games::delete_games)
    })
        .bind("0.0.0.0:8080")
        .expect("Failed to bind to port")
        .run()
        .await
}
