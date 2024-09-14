use actix_web::web::Json;
use actix_web::{post, web, Error, HttpResponse, Responder};
use entity::user::ActiveModel;
use sea_orm::{ActiveModelTrait, ActiveValue};

use crate::users::models::{ApiResponse, UserRequest};
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
        // TODO: create `serializer.rs` for this
        // TODO: and use `https://github.com/waltzofpearls/dateparser/tree/main/dateparser`
        // TODO: to parse the dates
        last_login: ActiveValue::Set(None),
        date_joined: ActiveValue::Set(None),
        created_at: ActiveValue::Set(None),
        updated_at: ActiveValue::Set(None),
        ..Default::default()
    };

    let result = user.insert(&app_state.db).await;
    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}