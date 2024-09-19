# Updating the logger

When we first set up logging for the application, we used `std::env`. Actix web documentation suggests that we use
`env_logger`, <https://actix.rs/docs/middleware/#logging>. So, I updated the `set_logger` function as follows,

```rust
use env_logger::Env;

pub fn set_logger() {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
}
```

Our `main.rs` now looks like this,

```rust
mod utils;
mod home;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use utils::config::get_address;
use utils::log::set_logger;

fn init() {
    set_logger();
    dotenv::dotenv().ok();
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
```

We no longer call the `env_logger::init();` in our `init` function.
