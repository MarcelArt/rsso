use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};


pub mod base;
pub mod permission;
pub mod user;

#[derive(Clone)]
pub struct Combined {
    pub permission: permission::Repo,
    pub user: user::Repo,
}

impl Combined {
    pub fn new(db: Surreal<Client>) -> Self {
        let db = Arc::new(db);

        Self {
            permission: permission::Repo::new(Arc::clone(&db)),
            user: user::Repo::new(Arc::clone(&db)),
        }
    }
}