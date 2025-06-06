use std::sync::Arc;
use crate::{models::role::{Role, RoleDto}, repos::base::{ICreate, IRead, IUpdate, IDelete, IGetById}};
use surrealdb::{engine::remote::ws::Client, Surreal};

const ROLES: &str = "roles";

#[derive(Clone, ICreate, IRead, IUpdate, IDelete, IGetById)]
#[entity("Role")]
#[dto("RoleDto")]
pub struct Repo {
    db: Arc<Surreal<Client>>,
    table_name: String,
}

impl Repo {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self {
            db,
            table_name: ROLES.to_string(),
        }
    }
    
}