# Returning JSON response

We would like our request [handlers](https://actix.rs/docs/handlers/) to return JSON instead of plain text. We need to
define a struct that we will serialize into JSON and serve as the response of our handlers. Let's do that first.

In `src/home` add a `models.rs` file. Now, add `models.rs` to `src/home/mod.rs`,

```rust
// private modules
mod models;

// public modules
pub mod handlers;
pub mod urls;
```

We don't need the `models.rs` module to be public, no other modules will use it for now. We still need the struct and
its member(s) to be public in order to access it from other modules within the `home` module. Our `models.rs` looks like
this at this moment,

```rust
use serde::Serialize;

#[derive(Serialize)]
pub struct HomeResponse {
    pub message: String,
}
```

In our handlers, we can now instantiate this struct, serialize it to JSON, and return it as a response.

Here's the updated `handlers.rs`,

```rust
use crate::home::models::HomeResponse;

use actix_web::web::Json;
use actix_web::{get, web, Error, Responder};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> Result<impl Responder, Error> {
    let response = HomeResponse {
        message: format!("Hello {name}!")
    };

    Ok(Json(response))
}

#[get("/test")]
pub async fn test() -> Result<impl Responder, Error> {
    let response = HomeResponse {
        message: "Testing...".to_string()
    };

    Ok(Json(response))
}
```

And, that's it. Our handlers now return JSON responses instead of plain text.
