use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event as CEvent, KeyEvent};

const EVENT_TICK_RATE_MS: u64 = 200;

pub enum Event<I> {
    Input(I),
    Tick,
}

/// Spawns a thread that will poll for inputs and write them to a mpsc channel
/// Returns the receiver for this channel so that another thread can handle inputs
pub fn spawn_event_thread() -> mpsc::Receiver<Event<KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(EVENT_TICK_RATE_MS);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });

    rx
}
