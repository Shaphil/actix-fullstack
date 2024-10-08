use actix_web::web;
use crate::users::handlers;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/users")
            .service(handlers::create_user)
            .service(handlers::get_users)
            .service(handlers::get_user)
            .service(handlers::update_user)
            .service(handlers::update_user_full)
            .service(handlers::delete_user)
    );
}