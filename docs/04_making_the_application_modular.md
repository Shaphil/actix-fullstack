# Making the application Modular

We will now start to break up the app into modules. This helps with code organization. Who doesn't love beautifully
organized code?

Let's add a home directory to `src`. We'll add a `mod.rs`, `handlers.rs` and a `urls.rs` file. After adding these files,
our project should look like this,

```bash
├── Cargo.lock
├── Cargo.toml
├── .env
├── src
│   ├── home
│   │   ├── handlers.rs
│   │   ├── mod.rs
│   │   └── urls.rs
│   ├── main.rs
│   └── utils
│       ├── config.rs
│       ├── log.rs
│       └── mod.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
```

In `home/mod.rs`, add the module declaration for `handlers.rs` and `urls.rs`,

```rust
// src/home/mod.rs
pub mod handlers;
pub mod urls;
```

Here's what the `handlers.rs` file looks like,

```rust
use actix_web::{get, web, Responder};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/test")]
pub async fn test() -> impl Responder {
    "Testing...".to_string()
}
```

Note that both greet and test returns String for now. We'll change that soon.

Here's the contents of the `urls.rs` file,

```rust
use actix_web::web;
use crate::home::handlers;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/home")
                .service(handlers::greet)
                .service(handlers::test)
        );
}
```

Read more, <https://actix.rs/docs/application/#configure>

We have scoped both the `greet` and `test` handler under `/home`. So they will be available under `/home/hello/{name}`
and `/home/test` routes respectively.

Remember how we added the `greet()` handler to `main()` before? We added it as a service like this,

```rust
App::new()
.wrap(Logger::default ())
.service(greet)  // this was what we did before
```

Now, we will add the routes as a configuration. Here's the `main.rs` file,

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
    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();
    let (host, port) = get_address();
    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(home::urls::routes)  // this configures the `home` routes
    })
        .bind((host, port))?
        .run()
        .await
}
```

With that, we have successfully split the application code into modules. Go ahead and run the application with cargo run
from the project directory. The application should be available at <http://localhost:8080> or <http://127.0.0.1:8080>
and the handlers at `/home/hello/{name}` and `/home/test` respectively under `localhost` and `127.0.0.1`.
