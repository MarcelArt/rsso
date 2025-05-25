use actix_web::{delete, get, post, put, web::{self, Json}};

use crate::{error::Error, models::permission::{Permission, PermissionDto}, repos::{self, base::IRepo}};

#[post("/")]
pub async fn create(repo: web::Data<repos::Combined>, input: Json<PermissionDto>) -> Result<Json<Option<Permission>>, Error> {
    let permission = repo.permission.create(input.0).await?;

    Ok(Json(permission))
}

#[get("/")]
pub async fn read(repo: web::Data<repos::Combined>) -> Result<Json<Vec<Permission>>, Error> {
    let permissions = repo.permission.read().await?;

    Ok(Json(permissions))
}

#[put("/{id}")]
pub async fn update(repo: web::Data<repos::Combined>, id: web::Path<String>, input: Json<PermissionDto>) -> Result<Json<Option<Permission>>, Error> {
    let permission = repo.permission.update(id.into_inner(), input.0).await?;

    Ok(Json(permission))
}

#[delete("/{id}")]
pub async fn delete(repo: web::Data<repos::Combined>, id: web::Path<String>) -> Result<Json<Option<Permission>>, Error> {
    let permission = repo.permission.delete(id.into_inner()).await?;

    Ok(Json(permission))
}

#[get("/{id}")]
pub async fn get_by_id(repo: web::Data<repos::Combined>, id: web::Path<String>) -> Result<Json<Option<Permission>>, Error> {
    let permission = repo.permission.get_by_id(id.into_inner()).await?;

    Ok(Json(permission))
}