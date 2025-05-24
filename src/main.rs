use actix_web::{App, HttpServer};
use db::surreal::SurrealDbState;
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use routes::{permission, user};

mod handlers;
mod models;
mod routes;
mod db;
mod error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let port = dotenv!("PORT");
    let port = port.parse::<u16>().expect("PORT must be a number");

    let app_state = SurrealDbState::connect().await?;

    println!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(permission::setup_routes())
            .service(user::setup_routes())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
