use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{models::permission::{Permission, PermissionDto}, repos::base::{ICreate, IRead, IUpdate, IDelete, IGetById}};

// use super::base::IRepo;

const PERMISSIONS: &str = "permissions";

#[derive(Clone, ICreate, IRead, IUpdate, IDelete, IGetById)]
#[entity("Permission")]
#[dto("PermissionDto")]
pub struct Repo {
    db: Arc<Surreal<Client>>,
    table_name: String,
}

impl Repo {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self {
            db,
            table_name: PERMISSIONS.to_string(),
        }
    }
    
}

// impl IRepo<Permission, PermissionDto> for Repo {
//     fn new(db: Arc<Surreal<Client>>) -> Self {
//         Self {
//             db,
//         }
//     }

//     async fn create(&self, input: PermissionDto) -> Result<Option<Permission>, crate::error::Error> {
//         let permission = self.db.create(PERMISSIONS).content(input).await?;
//         Ok(permission)
//     }

//     async fn read(&self) -> Result<Vec<Permission>, crate::error::Error> {
//         let permissions = self.db.select(PERMISSIONS).await?;
//         Ok(permissions)
//     }

//     async fn update(&self, id: String, input: PermissionDto) -> Result<Option<Permission>, crate::error::Error> {
//         let permission = self.db.update((PERMISSIONS, &id)).content(input).await?;
//         Ok(permission)
//     }

//     async fn delete(&self, id: String) -> Result<Option<Permission>, crate::error::Error> {
//         let permission = self.db.delete((PERMISSIONS, &id)).await?;
//         Ok(permission)
//     }

//     async fn get_by_id(&self, id: String) -> Result<Option<Permission>, crate::error::Error> {
//         let permission = self.db.select((PERMISSIONS, &id)).await?;
//         Ok(permission)
//     }
// }

// #[derive(ICreate, IRead, IUpdate, IDelete, IGetById)]
// #[entity("Permission")]
// #[dto("PermissionDto")]
// pub struct PermissionRepo {
//     db: Arc<Surreal<Client>>,
//     table_name: String,
// }