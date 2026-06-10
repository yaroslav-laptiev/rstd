use crate::{
    domain::repo::tasks_repo::TaskRepo,
    model::{
        app::AppMode,
        error::AppError,
        task::{Project, Status, Task},
    },
};

pub struct AppState<'a> {
    pub tasks: Vec<Task>,
    pub projects: Vec<Project>,
    pub selected_status: Status,
    pub selected_index: usize,
    pub should_quit: bool,
    pub mode: AppMode,
    repo: &'a mut dyn TaskRepo,
}

impl<'a> AppState<'a> {
    pub fn new(repo: &'a mut dyn TaskRepo) -> AppState<'a> {
        let projects = repo.load_projects().expect("Failed to load projects");
        let tasks: Vec<Task> = repo.load_tasks().expect("Failed to load tasks");
        AppState {
            tasks: tasks,
            projects: projects,
            selected_status: Status::Backlog,
            selected_index: 0,
            should_quit: false,
            mode: AppMode::Board,
            repo,
        }
    }

    pub fn tasks_for_status(&self, status: &Status) -> Vec<&Task> {
        self.tasks.iter().filter(|t| &t.status == status).collect()
    }

    pub fn select_next_status(&mut self) {
        self.selected_status = self.selected_status.next();
        self.selected_index = 0;
    }

    pub fn select_prev_status(&mut self) {
        self.selected_status = self.selected_status.prev();
        self.selected_index = 0;
    }

    pub fn select_next_task(&mut self) {
        let tasks_len = self.tasks_for_status(&self.selected_status).len();
        if self.selected_index >= tasks_len {
            return;
        }
        self.selected_index =
            (self.selected_index + 1) % self.tasks_for_status(&self.selected_status).len();
    }

    pub fn select_prev_task(&mut self) {
        if self.selected_index == 0 {
            return;
        }
        self.selected_index =
            (self.selected_index - 1) % self.tasks_for_status(&self.selected_status).len();
    }

    pub fn move_task_to_column(&mut self, status: &Status) -> Result<(), AppError> {
        let tasks = self.tasks_for_status(&self.selected_status);
        if tasks.is_empty() {
            return Ok(());
        }
        let mut task = tasks[self.selected_index].clone();
        task.status = *status;
        self.repo
            .update_task(&task)
            .expect("Failed to update the task");
        self.tasks = self.repo.load_tasks().expect("Failed to update tasks list");
        self.selected_status = *status;
        if let Some(idx) = self
            .tasks_for_status(status)
            .iter()
            .position(|t| t.id == task.id)
        {
            self.selected_index = idx;
        }
        Ok(())
    }

    pub fn create_task(&mut self, task: &Task) {
        self.repo
            .insert_task(task)
            .expect("failed to create a task");
        self.tasks = self.repo.load_tasks().expect("Failed to update tasks list");
    }

    pub fn delete_task(&mut self) {
        let task = self.tasks_for_status(&self.selected_status)[self.selected_index].clone();
        self.repo.delete_task(&task).expect("failed to delete task");
        self.tasks = self.repo.load_tasks().expect("Failed to update tasks list");
        self.selected_index = 0;
    }

    pub fn switch_mode(&mut self) {
        match self.mode {
            AppMode::Board => self.mode = AppMode::NewTask,
            AppMode::NewTask => self.mode = AppMode::Board,
        }
    }
}

pub struct TaskModalState {
    pub description_in: String,
    pub project_id: i16,
    pub deadline_in: String,
    pub selected_field: TaskModalField,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskModalField {
    Description,
    Project,
    Deadline,
}

impl TaskModalState {
    pub fn new() -> Self {
        Self {
            description_in: String::new(),
            project_id: 0,
            deadline_in: String::new(),
            selected_field: TaskModalField::Description,
        }
    }

    pub fn clear(&mut self) {
        self.description_in = "".to_string();
        self.deadline_in = "".to_string();
        self.project_id = 0;
        self.selected_field = TaskModalField::Description;
    }

    pub fn select_next_field(&mut self) {
        self.selected_field = match self.selected_field {
            TaskModalField::Description => TaskModalField::Project,
            TaskModalField::Project => TaskModalField::Deadline,
            TaskModalField::Deadline => TaskModalField::Description,
        };
    }

    pub fn select_next_project(&mut self, projects: &[Project]) {
        if projects.is_empty() {
            self.project_id = 0;
            return;
        }

        let current_index = self.selected_project_index(projects).unwrap_or(0);
        let next_index = (current_index + 1) % projects.len();
        self.project_id = projects[next_index].id;
    }

    pub fn select_prev_project(&mut self, projects: &[Project]) {
        if projects.is_empty() {
            self.project_id = 0;
            return;
        }

        let current_index = self.selected_project_index(projects).unwrap_or(0);
        let prev_index = if current_index == 0 {
            projects.len() - 1
        } else {
            current_index - 1
        };
        self.project_id = projects[prev_index].id;
    }

    pub fn selected_project_id(&self, projects: &[Project]) -> i16 {
        if self.project_id != 0 {
            return self.project_id;
        }

        projects.first().map(|project| project.id).unwrap_or(0)
    }

    pub fn selected_project_index(&self, projects: &[Project]) -> Option<usize> {
        let selected_id = self.selected_project_id(projects);
        projects
            .iter()
            .position(|project| project.id == selected_id)
    }
}
