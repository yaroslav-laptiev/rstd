use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::*,
};

use super::common::build_controls_row;

use crate::{app::TaskModalState, model::app::TASK_MODAL_CONTROLS};

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
