use actix_web::{delete, get, post, put, web::{self, Json}};

use crate::{db::surreal::SurrealDbState, error::Error, models::permission::{Permission, PermissionDto}};

const PERMISSIONS: &str = "permissions";

#[post("/")]
pub async fn create(data: web::Data<SurrealDbState>, input: Json<PermissionDto>) -> Result<Json<Option<Permission>>, Error> {
    let permission = data.db.create(PERMISSIONS).content(input).await?;

    Ok(Json(permission))
}

#[get("/")]
pub async fn read(data: web::Data<SurrealDbState>) -> Result<Json<Vec<Permission>>, Error> {
    let permissions = data.db.select(PERMISSIONS).await?;

    Ok(Json(permissions))
}

#[put("/{id}")]
pub async fn update(data: web::Data<SurrealDbState>, id: web::Path<String>, input: Json<PermissionDto>) -> Result<Json<Option<Permission>>, Error> {
    let permission = data.db.update((PERMISSIONS, &*id)).content(input).await?;

    Ok(Json(permission))
}

#[delete("/{id}")]
pub async fn delete(data: web::Data<SurrealDbState>, id: web::Path<String>) -> Result<Json<Option<Permission>>, Error> {
    let permission = data.db.delete((PERMISSIONS, &*id)).await?;

    Ok(Json(permission))
}

#[get("/{id}")]
pub async fn get_by_id(data: web::Data<SurrealDbState>, id: web::Path<String>) -> Result<Json<Option<Permission>>, Error> {
    let permission = data.db.select((PERMISSIONS, &*id)).await?;

    Ok(Json(permission))
}