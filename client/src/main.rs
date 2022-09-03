use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ui::{UIError, UI};

mod events;
mod ui;

fn main() -> Result<(), UIError> {
    let receiver = events::create_event_thread();

    let mut ui = UI::from(receiver);

    ui.render_loop()?;

    Ok(())
}
