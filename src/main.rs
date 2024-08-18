mod utils;
mod home;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use utils::config::{get_address, get_db};
use utils::log::set_logger;
use utils::app_state::AppState;

fn init() {
    set_logger();
    dotenv::dotenv().ok();
    env_logger::init();
}

async fn setup_db() -> DatabaseConnection {
    let db_url = get_db();
    Database::connect(db_url).await.unwrap()
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    init();

    let (host, port) = get_address();
    println!("Server running at http://{}:{}", host, port);

    let db = setup_db().await;
    Migrator::up(&db, None).await.unwrap();

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