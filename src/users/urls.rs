use actix_web::middleware::from_fn;
use actix_web::web;
use crate::users::handlers;
use crate::auth::middlewares::authenticate;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/auth/users")
                .wrap(from_fn(authenticate))
                .service(handlers::create_user)
                .service(handlers::update_user)
                .service(handlers::update_user_full)
                .service(handlers::delete_user)
        )
        .service(
            web::scope("/users")
                .service(handlers::get_users)
                .service(handlers::get_user)
                .service(handlers::login)
        );
}