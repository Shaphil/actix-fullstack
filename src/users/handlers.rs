use actix_web::{post, web, Error, HttpResponse, Responder};
use actix_web::web::Json;
use sea_orm::{ActiveModelTrait, ActiveValue};
use entity::user::{ActiveModel, Model};

use crate::users::models::UserRequest;
use crate::utils::app_state::AppState;


#[post("/create")]
pub async fn create_user(payload: Json<UserRequest>, app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let user = ActiveModel {
        username: ActiveValue::Set(payload.username.clone()),
        firstname: ActiveValue::Set(payload.firstname.clone()),
        lastname: ActiveValue::Set(payload.lastname.clone()),
        email: ActiveValue::Set(String::from(payload.email.clone())),
        password: ActiveValue::Set(payload.password.clone()),
        is_active: ActiveValue::Set(Option::from(payload.is_active.clone())),
        last_login: ActiveValue::Set(None),
        date_joined: ActiveValue::Set(None),
        created_at: ActiveValue::Set(None),
        updated_at: ActiveValue::Set(None),
        ..Default::default()
    };

    let user: Model = user.insert(&app_state.db).await.unwrap();
    Ok(HttpResponse::Ok().json(user))
}