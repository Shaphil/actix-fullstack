use std::env;
use std::env::VarError;
use std::time::Duration;
use lazy_static::lazy_static;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

lazy_static! {
    pub static ref HOST: String = set_host();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_db();
    pub static ref SECRET: String = set_secret();
}

// application defaults
const _HOST: &str = "127.0.0.1";
const _PORT: u16 = 8080;

fn get_env(key: &str) -> Result<String, VarError> {
    dotenv::dotenv().ok();
    env::var(key)
}

fn set_host() -> String {
    let host = get_env("HOST").unwrap_or(_HOST.to_string());
    if host != _HOST { _HOST.to_string() } else { host }
}

fn set_port() -> u16 {
    // if env variable `PORT` doesn't exist, we set `port` to the value of `_PORT`
    // which holds the value `8080` of type `u16`
    let port = get_env("PORT").unwrap_or(_PORT.to_string());
    // if the environment variable `PORT` exists but contains garbage value or
    // a value that cannot be parsed as `u16`, we provide the default value `_PORT` again
    port.parse::<u16>().unwrap_or(_PORT)
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

fn set_secret() -> String {
    get_env("SECRET").unwrap()
}

pub fn get_secret() -> String {
    (*SECRET).clone()
}