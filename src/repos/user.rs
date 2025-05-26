use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::user::{User, UserDto};

use super::base::IRepo;

const USERS: &str = "users";

#[derive(Clone)]
pub struct Repo {
    db: Arc<Surreal<Client>>,
}

impl IRepo<User, UserDto> for Repo {
    fn new(db: Arc<Surreal<Client>>) -> Self {
        Self {
            db,
        }
    }

    async fn create(&self, input: UserDto) -> Result<Option<User>, crate::error::Error> {
        let query = "
            insert into users (username, email, password)
            values
            ($username, $email, crypto::argon2::generate('$password'))
        ";
        let user = self.db.query(query)
            .bind(("username", input.username))
            .bind(("email", input.email))
            .bind(("password", input.password))
            .await?
            .take(0)?;
        Ok(user)
    }

    async fn read(&self) -> Result<Vec<User>, crate::error::Error> {
        let users = self.db.select(USERS).await?;
        Ok(users)
    }

    async fn update(&self, id: String, input: UserDto) -> Result<Option<User>, crate::error::Error> {
        let user = self.db.update((USERS, &id)).content(input).await?;
        Ok(user)
    }

    async fn delete(&self, id: String) -> Result<Option<User>, crate::error::Error> {
        let user = self.db.delete((USERS, &id)).await?;
        Ok(user)
    }

    async fn get_by_id(&self, id: String) -> Result<Option<User>, crate::error::Error> {
        let user = self.db.select((USERS, &id)).await?;
        Ok(user)
    }   
}

impl Repo {
    pub async fn login(&self, username: String, password: String) -> Result<Option<User>, crate::error::Error> {
        let query = "
            select * from users
            where (username = $username or email = $username)
            and crypto::argon2::compare(password, $password)
        ";
        let user = self.db.query(query)
            .bind(("username", username))
            .bind(("password", password))
            .await?
            .take(0)?;
        Ok(user)
    }
}