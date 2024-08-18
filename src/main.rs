mod utils;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};
use utils::log::set_logger;


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    set_logger();
    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}