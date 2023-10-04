use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = crate::repository::schema::locations)]
pub struct Location {
    #[serde(default)]
    pub id: i32,
    pub state_name: String,
    pub state_abbreviation: String,
    pub city: String,
}
