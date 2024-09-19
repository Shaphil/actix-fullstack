# Initial Project Setup

We start by creating a new cargo project. Head over to where your project will live, and issue the
`cargo new <project_name>` command. Replace `<project_name>` with your desired project name.

For my case I ran,

```bash
cargo new actix-fullstack
```

You should have the following file structure,

```bash
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
```

This is just a "hello world" program in Rust. We are going to change that.
Head over to <https://crates.io/crates/actix-web> and add the code sample from the Example section to your `main.rs`.
After which your `main.rs` should look like this,

```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

Inside the project directory, type in the following,

```bash
cargo add actix-web
```

This should update your project `Cargo.toml` and the `dependencies` section should now look like this,

```toml
[dependencies]
actix-web = "4.9.0"
```

We are done with the basic application setup. Let's test this. Hit `cargo run` from the terminal. Your server should be
up and running and would be available at <http://localhost:8080>. Head over to <http://localhost:8080/hello/Name> and
you shold be greeted with a very warm welcome. Don't forget to replace `Name` with your name.
