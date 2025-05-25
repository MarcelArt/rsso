use actix_web::{delete, get, post, put, web::{self, Json}, HttpResponse};

use crate::{models::permission::PermissionDto, repos::{self, base::IRepo}};

#[post("/")]
pub async fn create(repo: web::Data<repos::Combined>, input: Json<PermissionDto>) -> HttpResponse {
    let permission = repo.permission.create(input.0).await;

    match permission {
        Err(e) =>  {
            return HttpResponse::InternalServerError().body(e.to_string());
        },
        Ok(None) => {
            return HttpResponse::InternalServerError().body("Failed to create permission");
        },
        Ok(Some(permission)) => {
            return HttpResponse::Created().json(permission);
        }
    }
}

#[get("/")]
pub async fn read(repo: web::Data<repos::Combined>) -> HttpResponse {
    let permissions = repo.permission.read().await;

    match permissions {
        Err(e) =>  {
            return HttpResponse::InternalServerError().body(e.to_string());
        },
        Ok(permissions) => {
            return HttpResponse::Ok().json(permissions);
        }
    }
}

#[put("/{id}")]
pub async fn update(repo: web::Data<repos::Combined>, id: web::Path<String>, input: Json<PermissionDto>) -> HttpResponse {
    let permission = repo.permission.update(id.into_inner(), input.0).await;

    match permission {
        Err(e) =>  {
            return HttpResponse::InternalServerError().body(e.to_string());
        },
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        },
        Ok(Some(permission)) => {
            return HttpResponse::Ok().json(permission);
        }
    }
}

#[delete("/{id}")]
pub async fn delete(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let permission = repo.permission.delete(id.into_inner()).await;

    match permission {
        Err(e) =>  {
            return HttpResponse::InternalServerError().body(e.to_string());
        },
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        },
        Ok(Some(permission)) => {
            return HttpResponse::Ok().json(permission);
        }
    }
}

#[get("/{id}")]
pub async fn get_by_id(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let permission = repo.permission.get_by_id(id.into_inner()).await;

    match permission {
        Err(e) =>  {
            return HttpResponse::InternalServerError().body(e.to_string());
        },
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        },
        Ok(Some(permission)) => {
            return HttpResponse::Ok().json(permission);
        }
    }
}