use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::*,
};

use super::common::build_controls_row;

use crate::{
    app::{TaskModalField, TaskModalState},
    model::{app::TASK_MODAL_CONTROLS, task::Project},
};

pub fn render_task_modal(frame: &mut Frame, projects: &[Project], state: &TaskModalState) {
    let global_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)]);
    let global_chunks = global_layout.split(frame.area());

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
    .block(
        Block::default()
            .title("Description")
            .borders(Borders::ALL)
            .border_style(field_border_style(
                state.selected_field == TaskModalField::Description,
            )),
    );

    let deadline_text = Paragraph::new(if state.deadline_in.is_empty() {
        "Enter deadline date (or leave empty)".to_string()
    } else {
        state.deadline_in.to_string()
    })
    .style(Style::new().white().bold())
    .block(
        Block::default()
            .title("Deadline")
            .borders(Borders::ALL)
            .border_style(field_border_style(
                state.selected_field == TaskModalField::Deadline,
            )),
    );

    let project_items: Vec<ListItem> = projects
        .iter()
        .map(|project| {
            let selected = state.selected_project_id(projects) == project.id;
            let title = if selected {
                format!("> {}", project.title)
            } else {
                format!("  {}", project.title)
            };
            let style = if selected {
                Style::default().fg(Color::Black).bg(Color::Green)
            } else {
                Style::new().white()
            };

            ListItem::new(title).style(style)
        })
        .collect();

    let project_list = List::new(project_items).block(
        Block::default()
            .title("Project")
            .borders(Borders::ALL)
            .border_style(field_border_style(
                state.selected_field == TaskModalField::Project,
            )),
    );

    let project_selector_height = (projects.len() as u16 + 2).clamp(3, 8);
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(project_selector_height),
            Constraint::Length(3),
        ])
        .split(rect);
    frame.render_widget(text, layout[0]);
    frame.render_widget(project_list, layout[1]);
    frame.render_widget(deadline_text, layout[2]);
    frame.render_widget(
        build_controls_row(TASK_MODAL_CONTROLS.iter()),
        global_chunks[1],
    );
}

fn field_border_style(active: bool) -> Style {
    if active {
        Style::new().blue()
    } else {
        Style::new().green()
    }
}
