use std::env;
use std::env::VarError;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOST: String = set_host();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_db();
}

/// get_env: get `value` for a `key` from `.env`
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

pub fn get_db() -> String {
    (*DATABASE_URL).clone()
}