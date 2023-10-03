use actix_web::{get, web, HttpResponse};

use crate::repository::database::Database;

#[get("/posts")]
async fn get_posts(db: web::Data<Database>) -> HttpResponse {
    let posts = db.get_posts();

    HttpResponse::Ok().json(posts)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_posts);
}
