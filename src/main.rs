use actix_web::{HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use mobc_redis::RedisConnectionManager;
use redis::Commands;

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
    tracing_subscriber::fmt::init();
    let client = mobc_redis::redis::Client::open("redis://0.0.0.0:6379").expect("Invalid redis url");
    let pool_manager = RedisConnectionManager::new(client);
    let pool = mobc::Pool::builder().build(pool_manager);
    let pool_as_sharable_data = Data::new(pool);
    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(Data::clone(&pool_as_sharable_data))
            .service(hello_world)
            .configure(api::games::game_routes)
    })
        .bind("0.0.0.0:8080")
        .expect("Failed to bind to port")
        .run()
        .await
}
