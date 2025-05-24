use actix_web::web;

use crate::handlers::permission;

pub fn setup_routes() -> actix_web::Scope {
    web::scope("/permissions")
        .service(permission::create)
        .service(permission::read)
        .service(permission::update)
        .service(permission::delete)
}