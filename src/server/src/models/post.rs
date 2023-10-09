use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::posts)]
pub struct Post {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub image_url: String,
    pub date_picture: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
    pub family: Option<String>,
    pub gender: Option<String>,
    pub specie: Option<String>,
    pub location: i32,
    pub locality: String,
    pub verified: bool,
    pub published_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(MultipartForm)]
struct NewPost {
    pub title: Text<String>,
    pub image_url: TempFile,
    pub date_picture: Option<Text<String>>,
    pub description: Option<Text<String>>,
    pub family: Option<Text<String>>,
    pub gender: Option<Text<String>>,
    pub specie: Option<Text<String>>,
    pub location: Text<String>,
    pub locality: Text<String>,
}
