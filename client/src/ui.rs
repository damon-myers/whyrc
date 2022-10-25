use std::{
    io::{self, Stdout},
    sync::mpsc::{self, Receiver},
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Terminal,
};

use crate::{events::Event, net::NetworkHandles};

use self::{helper::default_block, menu::Menu, view::View};
pub use state::*;

mod helper;
mod menu;
mod room_list;
mod state;
mod view;

pub struct UI {
    state: UiState,
    menu: Menu,
    event_receiver: Receiver<Event<KeyEvent>>,
    net_handles: NetworkHandles,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    active_view: View,
}

#[derive(Debug)]
pub enum UIError {
    EventError(mpsc::RecvError),
    IOError(io::Error),
}

impl From<mpsc::RecvError> for UIError {
    fn from(err: mpsc::RecvError) -> Self {
        UIError::EventError(err)
    }
}

impl From<io::Error> for UIError {
    fn from(err: io::Error) -> Self {
        UIError::IOError(err)
    }
}

impl UI {
    pub fn from(
        ui_state: UiState,
        event_receiver: Receiver<Event<KeyEvent>>,
        net_handles: NetworkHandles,
    ) -> Self {
        enable_raw_mode().expect("can enable raw mode in terminal");
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("can set terminal modes");
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).expect("can create terminal with crossterm backend");

        UI {
            state: ui_state,
            menu: Menu::default(),
            event_receiver,
            net_handles,
            terminal,
            active_view: View::RoomList,
        }
    }

    pub fn render_loop(&mut self) -> Result<(), UIError> {
        loop {
            // get inputs
            match self.event_receiver.recv()? {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        self.reset_terminal()?;
                        break;
                    }
                    _ => {}
                },
                Event::Tick => {}
            }

            // render
            self.terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(6)].as_ref())
                    .split(frame.size());

                let menu_block = default_block("Menu");
                frame.render_widget(menu_block, chunks[0]);
                self.menu.render(frame, chunks[0]);

                self.active_view.render(&mut self.state, frame, chunks[1]);

                // TODO: Each view should also have a render method that takes in chunks[1] and the frame and renders themselves
                // let main_block = Block::default().borders(Borders::ALL);
                // frame.render_widget(main_block, chunks[1]);
            })?;
        }

        Ok(())
    }

    fn reset_terminal(&mut self) -> Result<(), io::Error> {
        // restore terminal
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;

        Ok(())
    }
}
