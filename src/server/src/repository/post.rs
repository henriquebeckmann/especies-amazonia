use chrono::Utc;
use diesel::prelude::*;

use crate::models::post::Post;
use crate::repository::database::Database;
use crate::repository::schema::posts::dsl::*;

impl Database {
    pub fn get_posts(&self) -> Vec<Post> {
        posts.load::<Post>(&mut self.pool.get().unwrap())
            .expect("Error loading all posts")
    }

    pub fn get_posts_by_id(&self, post_id: &str) -> Option<Post> {
        let post = posts
            .find(post_id)
            .get_result::<Post>(&mut self.pool.get().unwrap())
            .expect("Error loading post by id");

        Some(post)
    }

    pub fn update_post_by_id(&self, post_id: &str, mut post: Post) -> Option<Post> {
        post.updated_at = Utc::now().naive_utc();
        let post = diesel::update(posts.find(post_id))
            .set(&post)
            .get_result::<Post>(&mut self.pool.get().unwrap())
            .expect("Error updating post by id");

        Some(post)
    }

    pub fn delete_post_by_id(&self, post_id: &str) -> Option<usize> {
        let count = diesel::delete(posts.find(post_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting post by id");

        Some(count)
    }
}
