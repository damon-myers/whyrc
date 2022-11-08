use std::{sync::mpsc, thread};

use crossterm::event::{self, Event as CEvent, KeyEvent};

const EVENT_TICK_RATE_MS: u64 = 200;

pub enum Event<I> {
    Input(I),
}

/// Spawns a thread that will poll for inputs and write them to a mpsc channel
/// Returns the receiver for this channel so that another thread can handle inputs
pub fn spawn_event_thread() -> mpsc::Receiver<Event<KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            match event::read() {
                Ok(CEvent::Key(key)) => {
                    tx.send(Event::Input(key)).expect("can send events");
                }
                Err(_) => return,
                Ok(_) => {} // unhandled FocusEvent etc
            }
        }
    });

    rx
}
