use crate::users::models::ApiResponse;
use crate::utils::auth::{Claims, JSONWebToken};
use crate::utils::config::get_secret;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::http::header::AUTHORIZATION;
use actix_web::middleware::Next;
use actix_web::{HttpMessage, HttpResponse};
use serde_json::json;

pub async fn authenticate(request: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = request.headers().get(AUTHORIZATION);
    match auth {
        None => {
            // let response = ApiResponse { message: "Missing authentication token".to_string() };
            log::error!("auth token NOT provided");
            Err(AuthenticationError::MissingToken.into())
        }
        Some(header) => {
            let token = header.to_str().map_err(|_| AuthenticationError::InvalidTokenFormat(Default::default()))?;
            let token = token.replace("Bearer ", "").to_owned();
            let jwt = JSONWebToken { secret: get_secret() }; // Consider secure key access
            let claims = jwt.decode(token);
            // log::info!("auth token provided: {}", token);
            match claims {
                Ok(data) => {
                    let claims_info = Claims {
                        exp: data.claims.exp,
                        iat: data.claims.iat,
                        id: data.claims.id,
                        email: data.claims.email,
                    };
                    request.extensions_mut().insert(claims_info);
                    next.call(request).await
                }
                Err(err) => {
                    log::error!("Error decoding token: {}", err);
                    Err(AuthenticationError::InvalidToken(err).into())
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum AuthenticationError {
    MissingToken,
    InvalidTokenFormat(std::fmt::Error),
    InvalidToken(jsonwebtoken::errors::Error),
}

impl From<AuthenticationError> for Error {
    fn from(err: AuthenticationError) -> Error {
        match err {
            AuthenticationError::MissingToken => {
                let response = ApiResponse { message: "Missing authentication token".to_string() };
                ErrorInternalServerError(json!(response)).into()
            }
            AuthenticationError::InvalidTokenFormat(err) => {
                let response = ApiResponse { message: format!("Invalid token format: {}", err) };
                ErrorInternalServerError(json!(response)).into()
            }
            AuthenticationError::InvalidToken(err) => {
                let response = ApiResponse { message: format!("Invalid token: {}", err) };
                ErrorInternalServerError(json!(response)).into()
            }
        }
    }
}