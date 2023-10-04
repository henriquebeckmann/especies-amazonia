use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

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
