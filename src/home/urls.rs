use actix_web::web;
use crate::home::handlers;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/home")
                .service(handlers::greet)
                .service(handlers::test)
        );
}