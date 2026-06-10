use crate::model::{
    error::AppError,
    task::{Project, Task},
};

pub trait TaskRepo {
    fn load_tasks(&self) -> Result<Vec<Task>, AppError>;

    fn insert_task(&mut self, t: &Task) -> Result<(), AppError>;

    fn update_task(&mut self, t: &Task) -> Result<(), AppError>;

    fn delete_task(&mut self, t: &Task) -> Result<(), AppError>;

    fn load_projects(&mut self) -> Result<Vec<Project>, AppError>;

    fn insert_project(&mut self, p: &Project) -> Result<(), AppError>;

    fn delete_project(&mut self, p: &Project) -> Result<(), AppError>;
}
