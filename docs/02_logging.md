## Logging

Now we will enable logging. Create a `utils` directory inside your cargo project. Inside this directory create two new
files, `mod.rs` and `log.rs`. Now your project should look like this,

```bash
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── main.rs
│   └── utils
│       ├── log.rs
│       └── mod.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
```

Inside `log.rs` add the following,

```rust
pub fn set_logger() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
}
```

To add this logger to our application, we need to register the logger as an app-wide middleware. To do this, we `wrap()`
the `Logger` in our application builder, like this,

```rust
App::new()
.wrap(Logger::default ())
.service(greet)
```

Now our `main.rs` should look like this,

```rust
mod utils;  // the `utils` module

use actix_web::middleware::Logger;  // the `Logger` middleware
use actix_web::{get, web, App, HttpServer, Responder};
use utils::log::set_logger;  // the `set_logger` fnction we defined earlier

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_logger();
    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())  // this is where we wrap the `Logger`
            .service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

Now our app should be logging everything to the console. To test this, run the server, grab your favorite browser or a
rest client (like Postman/Insomnia, etc), and hit the endpoint <http://localhost:8080/hello/user> with a `GET` request.
You should be greeted with a heartwarming response like this,

```text
Hello user!
```

and also see something like this in the console/terminal in which the Actix-Web application is running,

```bash
[2024-08-19T15:00:14Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET /hello/user HTTP/1.1" 200 11 "-" "PostmanRuntime/7.41.1" 0.000206
```

Great! We have successfully added Logging to our application.
