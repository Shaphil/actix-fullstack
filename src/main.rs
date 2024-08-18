mod utils;
mod home;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use utils::config::get_address;
use utils::log::set_logger;

fn init() {
    set_logger();
    dotenv::dotenv().ok();
    env_logger::init();
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    init();
    let (host, port) = get_address();
    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(home::urls::routes)
    })
        .bind((host, port))?
        .run()
        .await
}