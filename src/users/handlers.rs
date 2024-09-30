use crate::users::models::{ApiResponse, UserRequest};
use crate::users::serializers::UserSerializer;
use crate::utils::app_state::AppState;
use actix_web::web::Json;
use actix_web::{get, patch, post, web, Error, HttpResponse, Responder};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use entity::user::Entity as User;
use sea_orm::{ActiveModelTrait, EntityTrait};

#[post("/create")]
pub async fn create_user(payload: Json<UserRequest>, app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let serializer = UserSerializer { data: payload };
    let user = serializer.serialize();

    let result = user.insert(&app_state.db).await;
    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

#[get("")]
pub async fn get_users(app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let result = User::find().all(&app_state.db).await;
    match result {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

#[get("/{id}")]
pub async fn get_user(id: web::Path<i32>, app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let user_id = id.into_inner();
    let result = User::find_by_id(user_id.clone())
        .one(&app_state.db)
        .await;

    match result {
        Ok(model) => {
            match model {
                None => {
                    let message = format!("User with ID `{}`, does not exist", user_id);
                    let response = ApiResponse { message };
                    Ok(HttpResponse::NotFound().json(response))
                }
                Some(user) => Ok(HttpResponse::Ok().json(user))
            }
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

#[patch("/{id}")]
pub async fn update_user(id: web::Path<i32>, payload: Json<UserRequest>, app_state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let user_id = id.into_inner();
    let result = User::find_by_id(user_id.clone()).one(&app_state.db).await;

    match result {
        Ok(model) => {
            match model {
                None => {
                    let message = format!("User with ID `{}`, does not exist", user_id.clone());
                    let response = ApiResponse { message };
                    Ok(HttpResponse::NotFound().json(response))
                }
                Some(mut user) => {
                    user.username = payload.username.clone().or(user.username);
                    user.firstname = payload.firstname.clone().or(user.firstname);
                    user.lastname = payload.lastname.clone().or(user.lastname);
                    user.email = payload.email.clone().or(user.email);
                    user.password = payload.password.clone().or(user.password);
                    user.is_active = Option::from(payload.is_active).or(user.is_active);
                    user.is_admin = Option::from(payload.is_admin).or(user.is_admin);
                    user.is_superadmin = Option::from(payload.is_superadmin).or(user.is_superadmin);
                    user.updated_at = Option::from(
                        NaiveDateTime::new(
                            NaiveDate::from(Utc::now().naive_utc()),
                            NaiveTime::from(Utc::now().time()),
                        )
                    );

                    Ok(HttpResponse::Ok().json(user))
                }
            }
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}