use crate::{
    data::src::tasks::TasksDataSrc,
    domain::repo::tasks_repo::TaskRepo,
    model::{
        error::AppError,
        task::{Project, Task},
    },
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

    fn load_projects(&mut self) -> Result<Vec<Project>, AppError> {
        self.list_projects()
    }

    fn insert_project(&mut self, p: &crate::model::task::Project) -> Result<(), AppError> {
        todo!()
    }

    fn delete_project(&mut self, p: &crate::model::task::Project) -> Result<(), AppError> {
        todo!()
    }
}
