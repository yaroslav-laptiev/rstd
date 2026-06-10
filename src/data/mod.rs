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
        self.ensure_project_id_column();

        Ok(())
    }

    fn ensure_project_id_column(&mut self) {
        let columns = {
            let mut stmt = self
                .connection
                .prepare("PRAGMA table_info(tasks)")
                .expect("Failed to inspect tasks table");

            stmt.query_map([], |row| row.get::<_, String>("name"))
                .expect("Failed to read tasks table columns")
                .collect::<Result<Vec<_>, _>>()
                .expect("Failed to read tasks table column name")
        };

        if columns.iter().any(|name| name == "project_id") {
            return;
        }

        self.connection
            .execute(
                "ALTER TABLE tasks ADD COLUMN project_id INTEGER NOT NULL DEFAULT 0",
                [],
            )
            .expect("Failed to add project_id to tasks table");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_can_be_applied_repeatedly_and_add_project_id() {
        let mut db = Database {
            connection: Connection::open_in_memory().expect("Failed to open test database"),
        };

        db.apply_migrations().expect("First migration pass failed");
        db.apply_migrations().expect("Second migration pass failed");

        let columns = {
            let mut stmt = db
                .connection
                .prepare("PRAGMA table_info(tasks)")
                .expect("Failed to inspect tasks table");

            stmt.query_map([], |row| row.get::<_, String>("name"))
                .expect("Failed to read tasks table columns")
                .collect::<Result<Vec<_>, _>>()
                .expect("Failed to read tasks table column name")
        };

        assert!(columns.iter().any(|name| name == "project_id"));
    }
}
