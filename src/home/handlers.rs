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