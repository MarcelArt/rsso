use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize)]
pub struct Role {
    pub id: RecordId,
    pub value: String,
    pub permissions: Vec<RecordId>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleDto {
    pub value: String,
    pub permissions: Vec<RecordId>,
}