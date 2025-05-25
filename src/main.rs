use actix_web::{App, HttpServer};
use db::surreal::{self};
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use routes::{permission, user};

mod handlers;
mod models;
mod routes;
mod db;
mod error;
mod repos;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let port = dotenv!("PORT");
    let port = port.parse::<u16>().expect("PORT must be a number");

    let db = surreal::connect().await?;
    let repos = repos::Combined::new(db);

    println!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(repos.clone())
            .service(permission::setup_routes())
            .service(user::setup_routes())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
