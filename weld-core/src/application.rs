use window::WebrenderWindow;
use events::{Event, EventStream};

pub struct Application {
    window: WebrenderWindow,
    events: EventStream
}

impl Application {
    pub fn new(title: &'static str) -> Application {
        Application {
            window: WebrenderWindow::new(title),
            events: EventStream::new()
        }
    }

    pub fn run(&self) {
        let window_join_handle = self.window.start_thread(self.events.sender());

        loop {
            let event = self.events.receiver().recv().unwrap();
            match event {
                Event::ApplicationClosed => break,
                _ => ()
            }
        }

        let _ = window_join_handle.join();
    }

    pub fn window(&self) -> &WebrenderWindow {
        &self.window
    }
}