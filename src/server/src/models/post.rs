use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::post)]
pub struct Post {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub imageurl: String,
    pub datepicture: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
    pub family: Option<String>,
    pub gender: Option<String>,
    pub specie: Option<String>,
    pub location: i32,
    pub locality: String,
    pub verified: bool,
    pub publishedat: chrono::NaiveDateTime,
}
