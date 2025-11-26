use crate::model::{error::AppError, task::Task};

pub trait TaskRepo {
    fn load_tasks(&self) -> Result<Vec<Task>, AppError>;

    fn insert_task(&mut self, t: &Task) -> Result<(), AppError>;

    fn update_task(&mut self, t: &Task) -> Result<(), AppError>;

    fn delete_task(&mut self, t: &Task) -> Result<(), AppError>;
}
