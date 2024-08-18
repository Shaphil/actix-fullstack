use actix_web::{get, web, Responder};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/test")]
pub async fn test() -> impl Responder {
    "Testing...".to_string()
}