use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod controllers;
mod models;
mod repository;

use controllers::{post, healthcheck, not_found};
use repository::database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = Database::new(database_url).pool();
    let app_data = web::Data::new(db_pool);

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .service(healthcheck)
            .service(
                web::scope("/api/v1")
                    .configure(post::config)
            )
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
