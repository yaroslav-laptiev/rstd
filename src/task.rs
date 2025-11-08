use chrono::{DateTime, Local};
use strum::{EnumIter, EnumMessage};
use strum_macros::{Display, EnumString};

#[derive(Display, EnumString, EnumMessage, Debug, Clone, Copy, PartialEq, EnumIter)]
#[repr(u8)]
pub enum Status {
    #[strum(serialize = "backlog", message = "BACKLOG")]
    Backlog = 0,
    #[strum(serialize = "today", message = "TODAY")]
    Today = 1,
    #[strum(serialize = "in_progress", message = "IN PROGRESS")]
    InProgress = 2,
    #[strum(serialize = "done", message = "DONE")]
    Done = 3,
    #[strum(serialize = "archived", message = "ARCHIVED")]
    Archived = 4,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Option<i64>,
    pub status: Status,
    pub description: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deadline: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(
        description: String,
        status: Option<Status>,
        deadline: Option<DateTime<Local>>,
    ) -> Task {
        Task {
            id: None,
            status: status.unwrap_or(Status::Backlog),
            description,
            created_at: Local::now(),
            updated_at: Local::now(),
            deadline,
        }
    }
}

impl Status {
    const VARIANTS: [Status; 5] = [
        Status::Backlog,
        Status::Today,
        Status::InProgress,
        Status::Done,
        Status::Archived,
    ];

    pub fn next(self) -> Self {
        let curr_idx = self as usize;
        let next_idx = (curr_idx + 1) % Self::VARIANTS.len();
        Self::VARIANTS[next_idx]
    }

    pub fn prev(self) -> Self {
        let curr_idx = self as usize;
        if curr_idx == 0 {
            return Self::VARIANTS[Self::VARIANTS.len() - 1];
        }
        let next_idx = (curr_idx - 1) % Self::VARIANTS.len();
        Self::VARIANTS[next_idx]
    }
}
