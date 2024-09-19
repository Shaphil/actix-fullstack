# Reading application defaults from .env

See the `bind()` method? At this moment we have the server address and port hardcoded to it,
`.bind(("127.0.0.1", 8080))`. Let's put them into a`.env` file where they belong, and read the values from there.
Our`.env` file should look like this,

```toml
HOST = 127.0.0.1
PORT = 8080
```

To use these settings from the `.env` file and make them available to our application we need some additional setup.
Let's add a necessary dependency,

```bash
cargo add lazy_static
```

this will add `lazy_static = "1.5.0"` to our `dependencies`.

With that, let's add the following code to `src/utils/config.rs`,

```rust
use std::env;
use std::env::VarError;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOST: String = set_host();
    pub static ref PORT: u16 = set_port();
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

// we'll be using this fnction soon
pub fn get_address() -> (String, u16) {
    let host = (*HOST).clone();
    let port = (*PORT).clone();

    (host, port)
}
```

With this, we can get the values of the variables `HOST` and `PORT` defined in our`.env`.

To use the values in our application, let's update our `main.rs` like this,

```rust
mod utils;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};
use utils::log::set_logger;
use utils::config::get_address;  // this is the function we just wrote

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

fn init() {
    set_logger();
    dotenv::dotenv().ok();
    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();
    let (host, port) = get_address();  // destructre the values

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(greet)
    })
        .bind((host, port))?  // replace the hardcoded ones
        .run()
        .await
}
```

This should do the trick. Oh and don't forget to update the `mod.rs` file in the utils module.

```rust
// src/utils/mod.rs
pub mod log;
pub mod config;
```

With this setup, we can now easily set any hardcoded value from our application as an environment variable and read/load
the values from there as needed. This will come in handy when we add the database connection string later in our
application. We don't want to hardcode the database connection in our application code and we don't want to leak it to
others. That's why we keep sensitive information like these in a`.env` file and add it to`.gitignore`.
