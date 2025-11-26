mod app;
mod data;
mod domain;
mod model;
mod ui;
mod utils;

use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use crate::{
    app::{AppState, TaskModalState},
    data::{Database, src::tasks::TasksDataSrc},
    model::{app::AppMode, error::AppError, task::Task},
    ui::{board::render_board, new_task::render_task_modal},
    utils::dt_utils::str_to_local_dt,
};

fn main() -> Result<(), AppError> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let mut db = Database::new().expect("Failed to connect to DB");
    db.apply_migrations().expect("Failed to apply migrations");

    let mut task_ds = TasksDataSrc::new(db);
    let mut app = AppState::new(&mut task_ds);
    let mut modal_state = TaskModalState::new();

    loop {
        match app.mode {
            AppMode::Board => {
                let _ = terminal.draw(|frame| {
                    render_board(frame, &app);
                })?;
            }
            AppMode::NewTask => {
                let _ = terminal.draw(|frame| {
                    render_task_modal(frame, &modal_state);
                })?;
            }
        }

        loop {
            if let Event::Key(key_event) = event::read()? {
                match app.mode {
                    AppMode::Board => match key_event.code {
                        KeyCode::Char('q') => {
                            app.should_quit = true;
                            break;
                        }
                        KeyCode::Tab => {
                            app.select_next_status();
                            break;
                        }
                        KeyCode::BackTab => {
                            app.select_prev_status();
                            break;
                        }
                        KeyCode::Left => {
                            app.move_task_to_column(&app.selected_status.prev())?;
                            break;
                        }
                        KeyCode::Right => {
                            app.move_task_to_column(&app.selected_status.next())?;
                            break;
                        }
                        KeyCode::Down => {
                            app.select_next_task();
                            break;
                        }
                        KeyCode::Up => {
                            app.select_prev_task();
                            break;
                        }
                        KeyCode::Char('n') => {
                            app.switch_mode();
                            break;
                        }
                        KeyCode::Char('d') => {
                            app.delete_task();
                            break;
                        }
                        _ => continue,
                    },
                    AppMode::NewTask => match (key_event.code, key_event.modifiers) {
                        (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                            let deadline = str_to_local_dt(&modal_state.deadline_in);

                            let task =
                                Task::new(modal_state.description_in.to_string(), None, deadline);
                            app.create_task(&task);
                            app.switch_mode();
                            modal_state.clear();
                            break;
                        }
                        (KeyCode::Backspace, _) => {
                            if !modal_state.entering_deadline {
                                modal_state.description_in.pop();
                            } else {
                                modal_state.deadline_in.pop();
                            }
                            break;
                        }
                        (KeyCode::Esc, _) => {
                            app.switch_mode();
                            modal_state.clear();
                            break;
                        }
                        (KeyCode::Char(_), _) => {
                            if let Some(ch) = key_event.code.as_char() {
                                if !modal_state.entering_deadline {
                                    modal_state.description_in.push(ch);
                                } else {
                                    modal_state.deadline_in.push(ch);
                                }
                                break;
                            }
                        }
                        (KeyCode::Enter, _) => {
                            if !modal_state.entering_deadline {
                                modal_state.description_in.push('\n');
                            }
                        }
                        (KeyCode::Tab, _) => {
                            modal_state.entering_deadline = !modal_state.entering_deadline;
                        }
                        _ => continue,
                    },
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    Ok(())
}
