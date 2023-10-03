use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database(ConnectionManager<PgConnection>);

impl Database {
    pub fn new(database_url: String) -> Self {
        let connection = ConnectionManager::<PgConnection>::new(database_url);
        Database(connection)
    }

    pub fn pool(self) -> DBPool {
        r2d2::Pool::builder()
            .build(self.0)
            .expect("Failed to create pool.")
    }
}
