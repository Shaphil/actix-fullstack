use crate::auth::models::RefreshToken;
use crate::utils::auth::JSONWebToken;
use crate::utils::config::get_secret;
use crate::utils::response::ApiResponse;

use actix_web::web::Json;
use actix_web::{post, Error, HttpResponse, Responder};


#[post("/refresh")]
pub async fn refresh_jwt(payload: Json<RefreshToken>) -> Result<impl Responder, Error> {
    let token = payload.token.clone();
    let jwt = JSONWebToken { secret: get_secret() };
    match jwt.decode(token) {
        Ok(data) => {
            let id = data.claims.id;
            let email = data.claims.email;
            let tokens = jwt.encode(id, email);
            Ok(HttpResponse::Ok().json(tokens))
        }
        Err(err) => {
            let response = ApiResponse { message: err.to_string() };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}