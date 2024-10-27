use crate::auth::handlers;
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/auth")
                .service(handlers::refresh_jwt)
        );
}