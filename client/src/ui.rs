use std::{
    io::{self, Stdout},
    sync::mpsc::{self, Receiver, Sender},
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
use whyrc_protocol::{ClientMessage, ServerMessage};

use crate::{events::Event, net::NetworkHandles};

use self::menu::Menu;

mod menu;
mod room_list;

pub struct UI {
    menu: Menu,
    event_receiver: Receiver<Event<KeyEvent>>,
    net_handles: NetworkHandles,
    terminal: Terminal<CrosstermBackend<Stdout>>,
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
    pub fn from(event_receiver: Receiver<Event<KeyEvent>>, net_handles: NetworkHandles) -> Self {
        enable_raw_mode().expect("can enable raw mode in terminal");
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("can set terminal modes");
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).expect("can create terminal with crossterm backend");

        UI {
            menu: Menu::default(),
            event_receiver,
            net_handles,
            terminal,
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

                let menu_block = Block::default().title("menu").borders(Borders::ALL);
                frame.render_widget(menu_block, chunks[0]);
                self.menu.render(frame, chunks[0]);

                let main_block = Block::default().borders(Borders::ALL);
                frame.render_widget(main_block, chunks[1]);
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
