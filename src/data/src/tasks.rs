use core::time;
use std::{cell::RefCell, str::FromStr};

use chrono::{DateTime, Local};
use rusqlite::params;

use crate::{
    data::Database,
    model::{
        error::AppError,
        task::{Project, Status, Task},
    },
    utils::dt_utils::*,
};

pub struct TasksDataSrc {
    db: RefCell<Database>,
}

impl TasksDataSrc {
    pub fn new(db: Database) -> Self {
        Self {
            db: RefCell::new(db),
        }
    }

    pub fn initialize(&self) -> Result<(), AppError> {

        let projects = self.load_projects()?;
        if projects.len() == 0 {
            // add new() func for Project type
            let def_project = Project {
                title: "Default".to_string(),
                created_at: chrono::Local::now(),
                updated_at: Local::now(),
                id: 0,
                status: "active".to_string(),
            };
            self.insert_project(&def_project)?;
        }
        Ok(())
    }

    pub fn load_projects(&self) -> Result<Vec<Project>, AppError> {
        let db = self.db.borrow();
        let mut stmnt = db.connection.prepare(r#"
            SELECT * FROM projects
        "#,)?;
        let projects_iter = stmnt.query_map([], |row| {
            
            let created_at_str: String = row.get("created_at")?;

            let created_at: DateTime<Local> = db_timestamp_to_local_dt(&created_at_str);

            let updated_at_str: String = row.get("updated_at")?;
            let updated_at = db_timestamp_to_local_dt(&updated_at_str);
            
            Ok(Project{
                id: row.get("id")?,
                status: row.get("status")?,
                title: row.get("title")?,
                created_at: created_at,
                updated_at: updated_at,
            })
        })?;
        let projects: Result<Vec<Project>, rusqlite::Error> = projects_iter.collect();
        Ok(projects?)
    }

    pub fn insert_project(&self, p: &Project) -> Result<(), AppError> {
           self.db.borrow().connection.execute(
            "INSERT INTO projects (title, status, created_at, updated_at)
VALUES (?1, ?2, ?3, ?4);",
            params![
                p.title,
                p.status.to_string(),
                p.created_at.to_rfc3339(),
                p.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn load_tasks(&self) -> Result<Vec<Task>, AppError> {
        let db = self.db.borrow();
        let mut stmt = db.connection.prepare("SELECT * FROM tasks")?;
        let task_iter = stmt.query_map([], |row| {
            let status_str: String = row.get("status")?;

            let created_at_str: String = row.get("created_at")?;

            let created_at: DateTime<Local> = db_timestamp_to_local_dt(&created_at_str);

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
                project_id: row.get("project_id")?,
            })
        })?;
        let tasks: Result<Vec<Task>, rusqlite::Error> = task_iter.collect();
        Ok(tasks?)
    }

    pub fn insert_task(&mut self, t: &Task) -> Result<(), AppError> {
        self.db.borrow().connection.execute(
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
            self.db.borrow().connection.execute(
                "UPDATE tasks SET updated_at = ?1, status = ?2 WHERE id = ?3",
                params![Local::now().to_rfc3339(), t.status.to_string(), id,],
            )?;
        }
        Ok(())
    }

    pub fn delete_task(&mut self, t: &Task) -> Result<(), AppError> {
        if let Some(id) = t.id {
            self.db
                .borrow()
                .connection
                .execute("DELETE from tasks WHERE id = ?1", params![id])?;
        }
        Ok(())
    }
}
