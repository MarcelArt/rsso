use actix_web::{get, post, web::{self, Json}};

use crate::{db::surreal::SurrealDbState, error::Error, models::user::{User, UserDto}};

const USERS: &str = "users";

#[get("/")]
pub async fn read(data: web::Data<SurrealDbState>) -> Result<Json<Vec<User>>, Error> {
    let users = data.db.select(USERS).await?;
    
    Ok(Json(users))
}

#[post("/")]
pub async fn create(data: web::Data<SurrealDbState>, user: Json<UserDto>) -> Result<Json<Option<User>>, Error> {
    let query = "
        insert into users (username, email, password)
        values
        ($username, $email, crypto::argon2::generate('$password'))
    ";
    
    let user = user.0;
    let user = data.db.query(query)
        .bind(("username", user.username))
        .bind(("email", user.email))
        .bind(("password", user.password))
        .await?
        .take(0)?;

    Ok(Json(user))
}