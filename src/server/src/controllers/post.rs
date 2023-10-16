use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::models::post::Post;
use crate::repository::database::Database;

#[get("/posts")]
async fn get_posts(db: web::Data<Database>) -> HttpResponse {
    let posts = db.get_posts();

    HttpResponse::Ok().json(posts)
}

#[get("/posts/{id}")]
async fn get_posts_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let post = db.get_posts_by_id(&id);

    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}

#[post("/posts")]
async fn create_post(db: web::Data<Database>, new_post: web::Json<Post>) -> HttpResponse {
    let post = db.create_post(new_post.into_inner());

    match post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/posts/{id}")]
async fn update_post_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
    updated_post: web::Json<Post>,
) -> HttpResponse {
    let post = db.update_post_by_id(&id, updated_post.into_inner());

    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}

#[delete("/posts/{id}")]
async fn delete_post_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let post = db.delete_post_by_id(&id);

    match post {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_posts);
    cfg.service(get_posts_by_id);
    cfg.service(update_post_by_id);
    cfg.service(delete_post_by_id);
}
