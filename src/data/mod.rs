pub mod repo;
pub mod src;
use rusqlite::Connection;

use crate::{model::error::AppError, utils::migrator::Migrator};

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Database, AppError> {
        if !std::fs::exists("./.rstd").unwrap_or(false) {
            std::fs::create_dir_all("./.rstd").expect("Failed to create data dir");
        }

        let connection = Connection::open("./.rstd/data.db")?;

        Ok(Database { connection })
    }

    pub fn apply_migrations(&mut self) -> Result<(), ()> {
        let migrations_str = Migrator::new().get_migrations();

        self.connection
            .execute_batch(&migrations_str)
            .expect("Failed to apply migrations");

        Ok(())
    }
}
