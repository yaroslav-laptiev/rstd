use crate::{
    data::src::tasks::TasksDataSrc,
    domain::repo::tasks_repo::TaskRepo,
    model::{error::AppError, task::Task},
};

impl TaskRepo for TasksDataSrc {
    fn load_tasks(&self) -> Result<Vec<Task>, AppError> {
        self.load_tasks()
    }

    fn insert_task(&mut self, t: &Task) -> Result<(), AppError> {
        self.insert_task(t)
    }

    fn update_task(&mut self, t: &Task) -> Result<(), AppError> {
        self.update_task(t)
    }

    fn delete_task(&mut self, t: &Task) -> Result<(), AppError> {
        self.delete_task(t)
    }
}
