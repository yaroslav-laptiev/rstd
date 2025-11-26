pub struct AppControl {
    pub key_binding: &'static str,
    pub title: &'static str,
}

pub const BOARD_CONTROLS: [AppControl; 6] = [
    AppControl {
        key_binding: "q",
        title: "Quit",
    },
    AppControl {
        key_binding: "Tab",
        title: "Select next column",
    },
    AppControl {
        key_binding: "←→",
        title: "Move task",
    },
    AppControl {
        key_binding: "↑↓",
        title: "Select task",
    },
    AppControl {
        key_binding: "d",
        title: "Delete task",
    },
    AppControl {
        key_binding: "n",
        title: "Create task",
    },
];

pub const TASK_MODAL_CONTROLS: [AppControl; 3] = [
    AppControl {
        key_binding: "Ctrl + s",
        title: "Create task",
    },
    AppControl {
        key_binding: "Tab",
        title: "Description/deadline switch",
    },
    AppControl {
        key_binding: "Esc",
        title: "Go back",
    },
];

pub enum AppMode {
    Board,
    NewTask,
}
