use ui::{UIError, UI};

mod events;
mod ui;

fn main() -> Result<(), UIError> {
    let receiver = events::spawn_event_thread();

    let mut ui = UI::from(receiver);

    ui.render_loop()?;

    Ok(())
}
