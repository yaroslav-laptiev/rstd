use std::slice::Iter;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::*,
};

use crate::model::app::AppControl;

pub fn build_controls_row(controls: Iter<AppControl>) -> Paragraph<'static> {
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
