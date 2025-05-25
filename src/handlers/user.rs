use actix_web::{get, post, web::{self, Json}, HttpResponse};

use crate::{models::{response::Response, user::{User, UserDto}}, repos::{self, base::IRepo}};

#[post("/")]
pub async fn create(repo: web::Data<repos::Combined>, user: Json<UserDto>) -> HttpResponse {    
    let user = user.0;
    let user = repo.user.create(user).await;

    match user {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<User>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::InternalServerError().json(Response::<User>::failure("Failed to create user".to_string()));
        },
        Ok(Some(u)) => {
            return HttpResponse::Created().json(Response::<User>::success(Some(u), "User created successfully".to_string()));
        }
    }
}

#[get("/")]
pub async fn read(repo: web::Data<repos::Combined>) -> HttpResponse {
    let users = repo.user.read().await;
    
    match users {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Vec<User>>::failure(e.to_string()));
        },
        Ok(us) => {
            return HttpResponse::Ok().json(Response::<Vec<User>>::success(Some(us), "Users retrieved successfully".to_string()));
        }
    }
}