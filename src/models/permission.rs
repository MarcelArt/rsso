use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize)]
pub struct Permission {
    pub id: RecordId,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct PermissionDto {
    pub name: String,
}