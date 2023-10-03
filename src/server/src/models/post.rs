use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = crate::repository::schema::post)]
pub struct Post {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub imageurl: String,
    pub datepicture: Option<chrono::NaiveDate>,
    pub description: String,
    pub family: String,
    pub gender: String,
    pub specie: String,
    pub location: i32,
    pub locality: String,
    pub verified: bool,
    pub publishedat: Option<chrono::NaiveDateTime>,
}
