use crate::users::models::{ApiResponse, UserRequest};
use crate::users::pagination::{Pagination, PaginationQuery};
use crate::users::serializers::UserSerializer;
use crate::utils::app_state::AppState;
use crate::utils::auth::JSONWebToken;
use crate::utils::config::get_secret;
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{delete, get, patch, post, put, Error, HttpResponse, Responder};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use entity::user::Column;
use entity::user::Entity as User;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};

#[post("/create")]
pub async fn create_user(payload: Json<UserRequest>, app_state: Data<AppState>) -> Result<impl Responder, Error> {
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

#[post("/login")]
pub async fn login(payload: Json<UserRequest>, app_state: Data<AppState>) -> Result<impl Responder, Error> {
    let result = User::find()
        .filter(Column::Username.eq(&payload.username.clone().unwrap()))
        .filter(Column::Password.eq(&payload.password.clone().unwrap()))
        .one(&app_state.db)
        .await;

    match result {
        Ok(user_option) => {
            match user_option {
                None => {
                    let response = ApiResponse { message: format!("User '{}' not found", &payload.username.clone().unwrap()) };
                    Ok(HttpResponse::NotFound().json(response))
                }
                Some(user) => {
                    let jwt = JSONWebToken { secret: get_secret() };
                    let token = jwt.encode(user.id, user.email.unwrap());
                    // TODO: this should be of type LoginResponse, not ApiResponse
                    let response = ApiResponse { message: token };
                    Ok(HttpResponse::Ok().json(response))
                }
            }
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}


#[get("")]
pub async fn get_users(query: Query<PaginationQuery>, app_state: Data<AppState>) -> Result<impl Responder, Error> {
    let page = Pagination { query: query.clone() };
    let user_pages = page.paginate();

    let pages = user_pages.all(&app_state.db).await;
    match pages {
        Ok(users) => {
            let result = page.response(users);
            Ok(HttpResponse::Ok().json(result))
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

#[get("/{id}")]
pub async fn get_user(id: Path<i32>, app_state: Data<AppState>) -> Result<impl Responder, Error> {
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
pub async fn update_user(id: Path<i32>, payload: Json<UserRequest>, app_state: Data<AppState>) -> Result<impl Responder, Error> {
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
                Some(user_model) => {
                    let mut user = user_model.into_active_model();
                    user.username = Set(payload.username.clone().or(user.username.unwrap()));
                    user.firstname = Set(payload.firstname.clone().or(user.firstname.unwrap()));
                    user.lastname = Set(payload.lastname.clone().or(user.lastname.unwrap()));
                    user.email = Set(payload.email.clone().or(user.email.unwrap()));
                    user.password = Set(payload.password.clone().or(user.password.unwrap()));
                    user.is_active = Set(Option::from(payload.is_active).or(user.is_active.unwrap()));
                    user.is_admin = Set(Option::from(payload.is_admin).or(user.is_admin.unwrap()));
                    user.is_superadmin = Set(Option::from(payload.is_superadmin).or(user.is_superadmin.unwrap()));
                    user.updated_at = Set(Option::from(
                        NaiveDateTime::new(
                            NaiveDate::from(Utc::now().naive_utc()),
                            NaiveTime::from(Utc::now().time()),
                        )
                    ));

                    let result = user.update(&app_state.db).await;
                    match result {
                        Ok(response) => Ok(HttpResponse::Ok().json(response)),
                        Err(err) => {
                            let response = ApiResponse { message: err.to_string() };
                            Ok(HttpResponse::BadRequest().json(response))
                        }
                    }
                }
            }
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

#[put("/{id}")]
pub async fn update_user_full(id: Path<i32>, payload: Json<UserRequest>, app_state: Data<AppState>) -> Result<impl Responder, Error> {
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
                Some(user_model) => {
                    let mut user = user_model.into_active_model();
                    user.username = Set(payload.username.clone());
                    user.firstname = Set(payload.firstname.clone());
                    user.lastname = Set(payload.lastname.clone());
                    user.email = Set(payload.email.clone());
                    user.password = Set(payload.password.clone());
                    user.is_active = Set(payload.is_active.clone());
                    user.is_admin = Set(payload.is_admin.clone());
                    user.is_superadmin = Set(payload.is_superadmin.clone());
                    user.updated_at = Set(Option::from(
                        NaiveDateTime::new(
                            NaiveDate::from(Utc::now().date_naive()),
                            NaiveTime::from(Utc::now().time()),
                        )
                    ));

                    let update = user.update(&app_state.db).await;
                    match update {
                        Ok(response) => Ok(HttpResponse::Ok().json(response)),
                        Err(err) => {
                            let response = ApiResponse { message: err.to_string() };
                            Ok(HttpResponse::BadRequest().json(response))
                        }
                    }
                }
            }
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

#[delete("/{id}")]
async fn delete_user(id: Path<i32>, app_state: Data<AppState>) -> Result<impl Responder, Error> {
    let user_id = id.into_inner();
    let result = User::find_by_id(user_id).one(&app_state.db).await;

    match result {
        Ok(model) => {
            match model {
                None => {
                    let message = format!("User with ID `{}`, does not exist", user_id.clone());
                    let response = ApiResponse { message };
                    Ok(HttpResponse::NotFound().json(response))
                }
                Some(user_model) => {
                    let user = user_model.into_active_model();
                    let res = user.delete(&app_state.db).await;

                    match res {
                        Ok(delete_result) => {
                            let message = format!("Deleted {} user with Id {}", delete_result.rows_affected, user_id.clone());
                            let response = ApiResponse { message };
                            Ok(HttpResponse::Ok().json(response))
                        }
                        Err(err) => {
                            let response = ApiResponse { message: err.to_string() };
                            Ok(HttpResponse::BadRequest().json(response))
                        }
                    }
                }
            }
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}