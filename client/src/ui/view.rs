mod room_list_view;

use std::io::Stdout;

use tui::{backend::CrosstermBackend, layout::Rect, Frame};

use super::{UIError, UiState};

pub use room_list_view::RoomListView;

pub trait View {
    fn render(
        &self,
        state: &mut UiState,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        area: Rect,
    ) -> Result<(), UIError>;

    fn will_handle_key_event(&self, event: crossterm::event::KeyEvent, state: &UiState) -> bool;

    fn handle_key_event(
        &mut self,
        event: crossterm::event::KeyEvent,
        state: &mut UiState,
        net_handles: &mut crate::net::NetworkHandles,
    );
}
