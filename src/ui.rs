use std::slice::Iter;

use crate::app::{AppControl, AppState, BOARD_CONTROLS, TASK_MODAL_CONTROLS, TaskModalState};
use ratatui::{
    Frame,
    layout::*,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::*,
};
use strum::{EnumMessage, IntoEnumIterator};

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
    for (i, s) in crate::task::Status::iter().enumerate() {
        let status = &s;
        frame.render_widget(build_column_for(app, status), table_chunks[i]);
    }
    frame.render_widget(build_controls_row(BOARD_CONTROLS.iter()), global_chunks[1]);
}

fn build_controls_row(controls: Iter<AppControl>) -> Paragraph<'static> {
    let spans: Vec<Span> = controls
        .map(|ac| {
            Span::styled(
                format!("[{}]: {}; ", ac.key_binding, ac.title),
                Style::default().fg(Color::Black),
            )
        })
        .collect();

    let l = Line::from(spans);

    Paragraph::new(l)
        .style(Style::default().bg(Color::Green))
        .centered()
}

fn build_column_for<'a>(app: &'a AppState, status: &crate::task::Status) -> List<'a> {
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

pub fn render_task_modal(frame: &mut Frame, state: &TaskModalState) {
    let global_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)]);
    let global_chunks = global_layout.split(frame.area());

    let block = Block::default()
        .title("Create New Task")
        .borders(Borders::ALL)
        .border_style(Style::new().green());
    let rect = Rect::new(
        global_chunks[0].width / 4,
        global_chunks[0].height / 4,
        global_chunks[0].width / 2,
        global_chunks[0].height / 2,
    );
    let text = Paragraph::new(if state.description_in.is_empty() {
        "Enter task description...".to_string()
    } else {
        state.description_in.to_string()
    })
    .style(Style::new().white().bold())
    .block(block);

    let deadline_text = Paragraph::new(if state.deadline_in.is_empty() {
        "Enter deadline date (or leave empty)".to_string()
    } else {
        state.deadline_in.to_string()
    })
    .style(Style::new().white().bold());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(80), // Backlog
            Constraint::Percentage(20), // Today
        ])
        .split(rect);
    frame.render_widget(text, layout[0]);
    frame.render_widget(deadline_text, layout[1]);
    frame.render_widget(
        build_controls_row(TASK_MODAL_CONTROLS.iter()),
        global_chunks[1],
    );
}
