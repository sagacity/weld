use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub enum Event {
    ApplicationClosed,
    Pressed
}

pub struct EventStream {
    tx: Sender<Event>,
    rx: Receiver<Event>
}

impl EventStream {
    pub fn new() -> EventStream {
        let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

        EventStream {
            tx,
            rx
        }
    }

    pub fn sender(&self) -> Sender<Event> {
        self.tx.clone()
    }

    pub fn receiver(&self) -> &Receiver<Event> {
        &self.rx
    }
}