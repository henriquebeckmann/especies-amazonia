use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = crate::repository::schema::location)]
pub struct Location {
    #[serde(default)]
    pub id: i32,
    pub state: String,
    pub acronym: String,
    pub country: String,
}
