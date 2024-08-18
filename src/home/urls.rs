use crate::home::handlers;

use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/home")
                .service(handlers::greet)
                .service(handlers::test)
        );
}