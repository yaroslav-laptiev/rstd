mod app;
mod db;
mod error;
mod migrator;
mod task;
mod ui;
mod utils;

use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use crate::{
    app::{AppMode, AppState, TaskModalState},
    db::Database,
    error::AppError,
    task::Task,
    ui::{render_board, render_task_modal},
};
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};

fn main() -> Result<(), AppError> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let mut db = Database::new().expect("Failed to connect to DB");
    db.apply_migrations().expect("Failed to apply migrations");

    let mut app = AppState::new(&db);
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
                        KeyCode::Left => {
                            app.move_task_to_column(&mut db, &app.selected_status.prev())?;
                            break;
                        }
                        KeyCode::Right => {
                            app.move_task_to_column(&mut db, &app.selected_status.next())?;
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
                            app.delete_task(&mut db);
                            break;
                        }
                        _ => continue,
                    },
                    AppMode::NewTask => match (key_event.code, key_event.modifiers) {
                        (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                            let deadline = crate::utils::str_to_local_dt(&modal_state.deadline_in);

                            let task =
                                Task::new(modal_state.description_in.to_string(), None, deadline);
                            app.create_task(&mut db, &task);
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
