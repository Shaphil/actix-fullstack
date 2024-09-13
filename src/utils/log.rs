use env_logger::Env;

pub fn set_logger() {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
}