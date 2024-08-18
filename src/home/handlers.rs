use actix_web::web::Json;
use actix_web::{get, web, Error, Responder};
use sea_orm::DatabaseBackend::Postgres;
use sea_orm::{ConnectionTrait, Statement};

use crate::home::models::HomeResponse;
use crate::utils::app_state::AppState;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> Result<impl Responder, Error> {
    let response = HomeResponse {
        message: format!("Hello {name}!")
    };

    Ok(Json(response))
}

#[get("/test")]
pub async fn test(app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    // db test
    let _res = app_state.db
        .query_all(Statement::from_string(Postgres, "select * from user;"))
        .await.unwrap();

    let response = HomeResponse {
        message: "Testing...".to_string()
    };

    Ok(Json(response))
}