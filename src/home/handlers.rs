use crate::home::models::{HomeResponse, UserRequest};
use crate::utils::app_state::AppState;
use actix_web::web::Json;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use entity::user::{ActiveModel, Model};
use sea_orm::DatabaseBackend::Postgres;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, Statement};
use chrono::Utc;

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

#[post("/create-user")]
pub async fn create_user(payload: Json<UserRequest>, app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let user = ActiveModel {
        user_name: ActiveValue::Set(payload.user_name.clone()),
        first_name: ActiveValue::Set(Option::from(payload.first_name.clone())),
        last_name: ActiveValue::Set(Option::from(payload.last_name.clone())),
        email: ActiveValue::Set(payload.email.clone()),
        is_active: ActiveValue::Set(payload.is_active.clone()),
        last_login: ActiveValue::Set(None),
        date_joined: ActiveValue::Set(Option::from(Utc::now().naive_utc())),
        password: ActiveValue::Set(payload.password.clone()),
        ..Default::default()
    };

    let user: Model = user.insert(&app_state.db).await.unwrap();

    // Ok(Json(user))
    Ok(HttpResponse::Ok().json(user))
}