// Utility methods for common UI elements
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn help_content<'a>() -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(" k, ↑  - move up one")]),
        Spans::from(vec![Span::raw(" j, ↓  - move down one")]),
        Spans::from(vec![Span::raw(" enter - join room")]),
        Spans::from(vec![Span::raw(" c     - create room")]),
        Spans::from(vec![Span::raw(" d     - delete room")]),
        Spans::from(vec![Span::raw(" q     - quit")]),
        Spans::from(vec![Span::raw("")]),
    ])
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Keybindings")
            .border_type(BorderType::Plain),
    )
}

pub fn default_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title(title)
        .border_type(BorderType::Plain)
}
