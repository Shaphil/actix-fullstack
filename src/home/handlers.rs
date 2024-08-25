use crate::home::models::{HomeResponse, UserRequest};
use crate::utils::app_state::AppState;
use actix_web::web::Json;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use entity::user::{ActiveModel, Model};
use sea_orm::DatabaseBackend::Postgres;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, Statement};
use sea_orm::prelude::DateTime;

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
    let date_format = "%Y-%m-%d %H:%M:%S";

    let last_login_str = payload.last_login.as_str();
    let last_login = DateTime::parse_from_str(last_login_str, date_format).unwrap();

    let date_joined_str = payload.date_joined.as_str();
    let date_joined = DateTime::parse_from_str(date_joined_str, date_format).unwrap();

    let user = ActiveModel {
        user_name: ActiveValue::Set(payload.user_name.clone()),
        first_name: ActiveValue::Set(Option::from(payload.first_name.clone())),
        last_name: ActiveValue::Set(Option::from(payload.last_name.clone())),
        email: ActiveValue::Set(payload.email.clone()),
        is_active: ActiveValue::Set(payload.is_active.clone()),
        last_login: ActiveValue::Set(Option::from(last_login)),
        date_joined: ActiveValue::Set(Option::from(date_joined)),
        password: ActiveValue::Set(payload.password.clone()),
        ..Default::default()
    };

    let user: Model = user.insert(&app_state.db).await.unwrap();

    // Ok(Json(user))
    Ok(HttpResponse::Ok().json(user))
}