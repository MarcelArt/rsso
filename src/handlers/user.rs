use actix_web::{get, post, web::{self, Json}};

use crate::{error::Error, models::user::{User, UserDto}, repos::{self, base::IRepo}};

#[get("/")]
pub async fn read(repo: web::Data<repos::Combined>) -> Result<Json<Vec<User>>, Error> {
    let users = repo.user.read().await?;
    
    Ok(Json(users))
}

#[post("/")]
pub async fn create(repo: web::Data<repos::Combined>, user: Json<UserDto>) -> Result<Json<Option<User>>, Error> {    
    let user = user.0;
    let user = repo.user.create(user).await?;

    Ok(Json(user))
}