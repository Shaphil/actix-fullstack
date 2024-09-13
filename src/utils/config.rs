use std::env;
use std::env::VarError;
use std::time::Duration;
use lazy_static::lazy_static;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

lazy_static! {
    pub static ref HOST: String = set_host();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_db();
}

fn get_env(key: &str) -> Result<String, VarError> {
    dotenv::dotenv().ok();
    env::var(key)
}

fn set_host() -> String {
    get_env("HOST").unwrap()
}

fn set_port() -> u16 {
    get_env("PORT").unwrap().parse::<u16>().unwrap()
}

pub fn get_address() -> (String, u16) {
    let host = (*HOST).clone();
    let port = (*PORT).clone();

    (host, port)
}

fn set_db() -> String {
    get_env("DATABASE_URL").unwrap()
}

pub async fn get_db_connection() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(DATABASE_URL.clone());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    Database::connect(opt).await.unwrap()
}