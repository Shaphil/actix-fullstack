mod utils;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};
use utils::log::set_logger;
use utils::config::get_address;


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

fn init() {
    set_logger();
    dotenv::dotenv().ok();
    env_logger::init();
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    init();
    let (host, port) = get_address();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(greet)
    })
        .bind((host, port))?
        .run()
        .await
}