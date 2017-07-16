use component::Component;
use window::{Epoch, WebrenderWindow};
use events::{Event, EventStream};
use std::sync::Arc;

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

    pub fn run(&mut self, root: Component) {
        let tree = Arc::new(root);
        let window_join_handle = self.window.start_thread(self.events.sender());

        loop {
            let mut epoch = Epoch(0);
            self.window.update(tree.clone(), &epoch);
            epoch.0 = epoch.0 + 1;

            let event = self.events.receiver().recv().unwrap();
            info!("Received application event: {:?}", event);
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