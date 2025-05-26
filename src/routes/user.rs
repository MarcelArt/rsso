use actix_web::web;

use crate::handlers::user;

pub fn setup_routes() -> actix_web::Scope {
    web::scope("/users")
        .service(user::create)
        .service(user::read)
        .service(user::login)
        .service(user::generate_new_token_pair)
}