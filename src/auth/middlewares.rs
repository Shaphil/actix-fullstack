use crate::utils::response::ApiResponse;
use crate::utils::auth::JSONWebToken;
use crate::utils::config::get_secret;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::Error;
use actix_web::http::header::AUTHORIZATION;
use actix_web::middleware::Next;
use actix_web::{HttpMessage, HttpResponse, ResponseError};
use std::fmt;

pub async fn authenticate(request: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = request.headers().get(AUTHORIZATION);
    match auth {
        None => {
            log::error!("auth token NOT provided");
            Err(AuthenticationError::MissingToken.into())
        }
        Some(header) => {
            let token = header.to_str().map_err(|_| AuthenticationError::InvalidTokenFormat(Default::default()))?;
            let token = token.replace("Bearer ", "").to_owned();
            let jwt = JSONWebToken { secret: get_secret() }; // Consider secure key access

            match jwt.decode(token) {
                Ok(data) => {
                    request.extensions_mut().insert(data.claims);
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
    InvalidTokenFormat(fmt::Error),
    InvalidToken(jsonwebtoken::errors::Error),
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthenticationError::MissingToken => write!(f, "Missing authentication token"),
            AuthenticationError::InvalidTokenFormat(err) => write!(f, "Invalid token format: {}", err),
            AuthenticationError::InvalidToken(err) => write!(f, "Invalid token: {}", err),
        }
    }
}

// Implement ResponseError for AuthenticationError
impl ResponseError for AuthenticationError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AuthenticationError::MissingToken => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthenticationError::InvalidTokenFormat(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AuthenticationError::InvalidToken(_) => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let api_response = match self {
            AuthenticationError::MissingToken => {
                ApiResponse { message: "Missing authentication token".to_string() }
            }
            AuthenticationError::InvalidTokenFormat(err) => {
                ApiResponse { message: format!("Invalid token format: {}", err) }
            }
            AuthenticationError::InvalidToken(err) => {
                ApiResponse { message: format!("Invalid token: {}", err) }
            }
        };

        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .body(serde_json::to_string(&api_response).unwrap()) // Convert to JSON string
    }
}
