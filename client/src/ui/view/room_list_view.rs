use std::io::Stdout;

use crossterm::event::{KeyCode, KeyEvent};
use protocol::ClientMessage;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use crate::ui::{
    helper::{default_block, help_content},
    UIError, UiState,
};

use super::View;

pub struct RoomListView<'a> {
    room_name_input: TextArea<'a>,
}

impl RoomListView<'_> {
    pub fn new() -> Self {
        let mut room_list_input = TextArea::default();

        room_list_input.set_cursor_line_style(Style::default());
        room_list_input.set_block(default_block("Enter room name:"));

        RoomListView {
            room_name_input: room_list_input,
        }
    }

    fn room_list(&self, state: &UiState) -> List {
        let list_items: Vec<ListItem> = state
            .room_list
            .room_names
            .iter()
            .map(|name| {
                ListItem::new(Spans::from(vec![Span::styled(
                    name.clone(),
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
}

impl View for RoomListView<'_> {
    fn render(
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

        // room_list_chunks[0] - actual room list or room name input field
        // room_list_chunks[1] - footer that shows room count and user count
        let room_list_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
            .split(room_list_page_chunks[1]);

        let help_content = help_content();

        let footer_content = format!(
            "There are {} room(s) and {} user(s) in the server.",
            state.room_list.room_names.len(),
            0 // TODO: Add users list to UiState
        );
        let footer = Paragraph::new(footer_content)
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(default_block("Server Stats"));

        frame.render_widget(help_content, room_list_page_chunks[0]);
        frame.render_widget(footer, room_list_chunks[0]);

        if state.room_list.is_creating_room {
            frame.render_widget(self.room_name_input.widget(), room_list_chunks[1])
        } else {
            let room_list = self.room_list(state);
            frame.render_stateful_widget(
                room_list,
                room_list_chunks[1],
                &mut state.room_list.room_list_state,
            );
        }

        Ok(())
    }

    fn handle_key_event(
        &mut self,
        event: crossterm::event::KeyEvent,
        state: &mut UiState,
        net_handles: &mut crate::net::NetworkHandles,
    ) {
        if state.room_list.is_creating_room {
            match event {
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    let room_name = self.room_name_input.lines().join("");
                    if room_name != "" {
                        net_handles
                            .sender
                            .send(ClientMessage::CreateRoom {
                                name: self.room_name_input.lines()[0].trim().to_string(), // only take the first line for the room name
                            })
                            .expect("can send messages to server");

                        // erase from cursor to beginning of line, should delete everything in the texarea
                        self.room_name_input.delete_line_by_head();

                        state.room_list.is_creating_room = false
                    }
                }
                event => {
                    self.room_name_input.input(event);
                }
            }
        } else {
            match event.code {
                KeyCode::Char('c') => state.room_list.is_creating_room = true,
                KeyCode::Char('d') => {
                    if state.room_list.room_names.is_empty() {
                        return;
                    }

                    let current = state
                        .room_list
                        .room_list_state
                        .selected()
                        .unwrap_or_default();

                    // guaranteed to exist because room_list_state stays within range of room_names indices
                    let current_room_name = &state.room_list.room_names[current];

                    net_handles
                        .sender
                        .send(ClientMessage::DeleteRoom {
                            name: current_room_name.to_string(),
                        })
                        .expect("can send messages to server");
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    let current = state
                        .room_list
                        .room_list_state
                        .selected()
                        .unwrap_or_default();

                    let new = std::cmp::min(
                        current + 1,
                        state.room_list.room_total_count.saturating_sub(1),
                    );

                    state.room_list.room_list_state.select(Some(new));
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    let current = state
                        .room_list
                        .room_list_state
                        .selected()
                        .unwrap_or_default();

                    let new = current.saturating_sub(1);

                    state.room_list.room_list_state.select(Some(new));
                }
                _ => {}
            }
        }
    }

    fn will_handle_key_event(&self, event: crossterm::event::KeyEvent, state: &UiState) -> bool {
        if state.room_list.is_creating_room {
            true
        } else {
            match event.code {
                KeyCode::Char('c') => true,
                KeyCode::Char('d') => true,
                KeyCode::Char('j') | KeyCode::Down => true,
                KeyCode::Char('k') | KeyCode::Up => true,
                _ => false,
            }
        }
    }
}
