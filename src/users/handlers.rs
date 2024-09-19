use actix_web::web::Json;
use actix_web::{get, patch, post, web, Error, HttpResponse, Responder};
use entity::user::Entity as User;
use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::users::models::{ApiResponse, UserRequest};
use crate::users::serializers::UserSerializer;
use crate::utils::app_state::AppState;


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
    let serializer = UserSerializer { data: payload };
    let updated_user = serializer.serialize();

    match result {
        Ok(model) => {
            match model {
                None => {
                    let message = format!("User with ID `{}`, does not exist", user_id.clone());
                    let response = ApiResponse { message };
                    Ok(HttpResponse::NotFound().json(response))
                }
                Some(mut user) => {
                    user.username = updated_user.username.unwrap();
                    user.firstname = updated_user.firstname.unwrap();
                    user.lastname = updated_user.lastname.unwrap();
                    user.email = updated_user.email.unwrap();
                    user.password = updated_user.password.unwrap();
                    user.is_active = updated_user.is_active.unwrap();
                    user.is_admin = updated_user.is_admin.unwrap();
                    user.is_superadmin = updated_user.is_superadmin.unwrap();

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