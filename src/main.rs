mod utils;
mod home;

use crate::utils::app_state::AppState;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use utils::config::{get_address, get_db_connection};
use utils::log::set_logger;

fn init() {
    set_logger();
    dotenv::dotenv().ok();
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    init();

    let db = get_db_connection().await;
    Migrator::up(&db, None).await.unwrap();

    let (host, port) = get_address();
    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(home::urls::routes)
    })
        .bind((host, port))?
        .run()
        .await
}