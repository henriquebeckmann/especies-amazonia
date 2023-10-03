use diesel::prelude::*;

use crate::models::post::Post;
use crate::repository::database::Database;
use crate::repository::schema::post::dsl::*;

impl Database {
    pub fn get_posts(&self) -> Vec<Post> {
        post.load::<Post>(&mut self.pool.get().unwrap())
            .expect("Error loading all posts")
    }
}
