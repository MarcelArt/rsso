use actix_web::{get, post, web::{self, Json}, HttpResponse};

use crate::{models::user::UserDto, repos::{self, base::IRepo}};

#[post("/")]
pub async fn create(repo: web::Data<repos::Combined>, user: Json<UserDto>) -> HttpResponse {    
    let user = user.0;
    let user = repo.user.create(user).await;

    match user {
        Err(e) =>  {
            return HttpResponse::InternalServerError().body(e.to_string());
        },
        Ok(None) => {
            return HttpResponse::InternalServerError().body("Failed to create user");
        },
        Ok(Some(u)) => {
            return HttpResponse::Created().json(u);
        }
    }
}

#[get("/")]
pub async fn read(repo: web::Data<repos::Combined>) -> HttpResponse {
    let users = repo.user.read().await;
    
    match users {
        Err(e) =>  {
            return HttpResponse::InternalServerError().body(e.to_string());
        },
        Ok(us) => {
            return HttpResponse::Ok().json(us);
        }
    }
}