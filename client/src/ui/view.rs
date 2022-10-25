use std::io::Stdout;

use protocol::Room;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::{
    helper::{default_block, help_content},
    UIError, UiState,
};

pub enum View {
    RoomList,
    RoomChat { name: String },
}

impl View {
    pub fn render(
        &self,
        state: &mut UiState,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        area: Rect,
    ) -> Result<(), UIError> {
        match &self {
            View::RoomList => self.render_room_list_view(state, frame, area),
            View::RoomChat { name } => self.render_room_view(state, frame, area, name),
        }
    }

    fn render_room_list_view(
        &self,
        state: &mut UiState,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        area: Rect,
    ) -> Result<(), UIError> {
        // room_list_page_chunks[0] - room list controls
        // room_list_page_chunks[1] - actual room list + footer
        let room_list_page_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(area);

        // room_list_chunks[0] - actual room list
        // room_list_chunks[1] - footer that shows room count and user count
        let room_list_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
            .split(room_list_page_chunks[1]);

        let help_content = help_content();

        let room_list = self.room_list(state);

        let footer_content = format!(
            "There are {} room(s) and {} user(s) in the server.",
            state.rooms.len(),
            0 // TODO: Add users list to UiState
        );
        let footer = Paragraph::new(footer_content)
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(default_block("Server Stats"));

        frame.render_widget(help_content, room_list_page_chunks[0]);
        frame.render_widget(footer, room_list_chunks[0]);
        frame.render_stateful_widget(room_list, room_list_chunks[1], &mut state.room_list_state);

        Ok(())
    }

    fn room_list(&self, state: &UiState) -> List {
        let list_items: Vec<ListItem> = state
            .rooms
            .iter()
            .map(|room| {
                ListItem::new(Spans::from(vec![Span::styled(
                    room.name.clone(),
                    Style::default(),
                )]))
            })
            .collect();

        List::new(list_items)
            .block(default_block("Room List"))
            .highlight_style(
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
    }

    fn render_room_view(
        &self,
        state: &UiState,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        area: Rect,
        room_name: &String,
    ) -> Result<(), UIError> {
        // left 20% is a guide to keyboard shortcuts for navigating the list
        // right 80% is a list of rooms from the UiState, highlighting and clicking on one will join it
        // need a stateful widget to track the highlighted room
        todo!()
    }
}
