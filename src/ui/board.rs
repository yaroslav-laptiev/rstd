use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::*,
};
use strum::{EnumMessage, IntoEnumIterator};

use crate::{
    app::AppState,
    model::{app::BOARD_CONTROLS, task::Status},
};

use super::common::build_controls_row;

pub fn render_board(frame: &mut Frame, app: &AppState) {
    let global_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)]);
    let global_chunks = global_layout.split(frame.area());
    let table_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Backlog
            Constraint::Percentage(20), // Today
            Constraint::Percentage(20), // InProgress
            Constraint::Percentage(20), // Done
            Constraint::Percentage(20), // Archived
        ])
        .split(global_chunks[0]);
    for (i, s) in Status::iter().enumerate() {
        let status = &s;
        frame.render_widget(build_column_for(app, status), table_chunks[i]);
    }
    frame.render_widget(build_controls_row(BOARD_CONTROLS.iter()), global_chunks[1]);
}

fn build_column_for<'a>(app: &'a AppState, status: &Status) -> List<'a> {
    let tasks: Vec<ListItem> = app
        .tasks_for_status(status)
        .iter()
        .enumerate()
        .map(|task| {
            let selected = app.selected_index == task.0 && app.selected_status == task.1.status;

            let dt_fmt = "%d/%m/%Y %H:%M";

            let fmt_data = format!(
                "{}{}\nCreatedAt: {}\nUpdated At: {}\n{}\n",
                if selected {
                    "> ".to_string()
                } else {
                    String::new()
                },
                task.1.description,
                &task.1.created_at.format(dt_fmt),
                &task.1.updated_at.format(dt_fmt),
                if let Some(deadline_str) = task.1.deadline {
                    format!("Due to: {}\n", deadline_str.format(dt_fmt)).to_string()
                } else {
                    String::new()
                }
            );

            if selected {
                ListItem::new(fmt_data).style(Style::new().blue().italic())
            } else {
                ListItem::new(fmt_data).style(Style::new().green().bold())
            }
        })
        .collect();
    List::new(tasks).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(if app.selected_status == *status {
                Style::new().blue()
            } else {
                Style::new().green()
            })
            .title(status.get_message().unwrap()),
    )
}
