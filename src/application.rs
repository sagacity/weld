use component::Component;
use window::{Epoch, RendererHandle, EventStream, WebrenderWindow};
use events::Event;
use std::sync::{Arc, Mutex};
use futures::Stream;
use tokio_core::reactor::Core;

pub struct Application {
    window: (RendererHandle, EventStream)
}

impl Application {
    pub fn new(title: &'static str) -> Application {
        Application {
            window: WebrenderWindow::new(title)
        }
    }

    pub fn run(self, root: Component) {
        let epoch = Epoch(0);
        let tree = Arc::new(Mutex::new(root));

        let (mut renderer, event_stream) = self.window;
        renderer.set_tree(tree.clone());
        renderer.render();

        let event_logger = event_stream.for_each(|event| {
            //println!("event: {:?}", event);
            match event {
                Event::NotifyRenderComplete => {
                    renderer.update();
                    Ok(())
                },
                Event::ApplicationClosed => {
                    //renderer.stop();
                    Err(())
                },
                _ => Ok(())
            }
        });

        let mut core = Core::new().unwrap();
        let _ = core.run(event_logger);
    }
}