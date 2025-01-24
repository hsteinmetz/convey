use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn create_root_layout(frame: &mut Frame, title: &str) -> Rc<[Rect]> {
    let layout: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ])
        .split(frame.area())
        .into();

    if let Some(first_section) = layout.get(0) {
        let title_widget = Paragraph::new(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default()),
        );

        frame.render_widget(title_widget, *first_section);
    }

    if let Some(last_section) = layout.get(2) {
        let footer_widget = Paragraph::new(Span::styled(
            "Footer",
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default()),
        );

        frame.render_widget(footer_widget, *last_section);
    }

    layout
}
