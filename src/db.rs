use std::str::FromStr;

use crate::{
    error::AppError,
    migrator::Migrator,
    task::{Status, Task},
    utils::db_timestamp_to_local_dt,
};
use chrono::{DateTime, Local};
use rusqlite::{Connection, params};

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

    pub fn load_tasks(&self) -> Result<Vec<Task>, AppError> {
        let mut stmt = self.connection.prepare("SELECT * FROM tasks")?;
        let task_iter = stmt.query_map([], |row| {
            let status_str: String = row.get("status")?;

            let created_at_str: String = row.get("created_at")?;

            let created_at = db_timestamp_to_local_dt(&created_at_str);

            let updated_at_str: String = row.get("updated_at")?;
            let updated_at = db_timestamp_to_local_dt(&updated_at_str);

            let maybe_deadline_str: Option<String> = row.get("deadline")?;
            let mut deadline: Option<DateTime<Local>> = None;
            if let Some(d_str) = maybe_deadline_str {
                deadline = Some(db_timestamp_to_local_dt(&d_str));
            }

            Ok(Task {
                id: row.get("id")?,
                description: row.get("description")?,
                status: Status::from_str(&status_str)
                    .map_err(|_| rusqlite::Error::UnwindingPanic)?,
                created_at: created_at,
                updated_at: updated_at,
                deadline: deadline,
            })
        })?;
        let tasks: Result<Vec<Task>, rusqlite::Error> = task_iter.collect();
        Ok(tasks?)
    }

    pub fn insert_task(&mut self, t: &Task) -> Result<(), AppError> {
        self.connection.execute(
            "INSERT INTO tasks (description, status, created_at, updated_at, deadline)
VALUES (?1, ?2, ?3, ?4, ?5);",
            params![
                t.description,
                t.status.to_string(),
                t.created_at.to_rfc3339(),
                t.updated_at.to_rfc3339(),
                t.deadline.as_ref().map(|d| d.to_rfc3339()),
            ],
        )?;
        Ok(())
    }

    pub fn update_task(&mut self, t: &Task) -> Result<(), AppError> {
        if let Some(id) = t.id {
            self.connection.execute(
                "UPDATE tasks SET updated_at = ?1, status = ?2 WHERE id = ?3",
                params![Local::now().to_rfc3339(), t.status.to_string(), id,],
            )?;
        }
        Ok(())
    }

    pub fn delete_task(&mut self, t: &Task) -> Result<(), AppError> {
        if let Some(id) = t.id {
            self.connection
                .execute("DELETE from tasks WHERE id = ?1", params![id])?;
        }
        Ok(())
    }
}
