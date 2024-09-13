use actix_web::web;
use crate::users::handlers;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/users")
            .service(handlers::create_user)
    );
}