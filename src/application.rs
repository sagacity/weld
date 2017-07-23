use component::Component;
use window::WebrenderWindow;
use events::{Event, Interaction};
use layout_context::LayoutContext;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use futures::Stream;
use tokio_core::reactor::Core;

pub struct Application {
    title: &'static str,
    layout_context: Rc<RefCell<LayoutContext>>,
}

impl Application {
    pub fn new(title: &'static str) -> Application {
        let layout_context = Rc::new(RefCell::new(LayoutContext::new()));

        Application {
            title,
            layout_context,
        }
    }

    pub fn run(self, root: Component) {
        //let epoch = Epoch(0);
        let tree = Arc::new(Mutex::new(root));

        let (mut renderer, event_stream) = WebrenderWindow::new(self.title, self.layout_context.clone());
        renderer.set_tree(tree.clone());
        renderer.render();

        let event_logger = event_stream.for_each(|event| {
            //println!("event: {:?}", event);
            match event {
                Event::Interaction(e) => {
                    self.handle_interaction(e, &tree.lock().unwrap());
                    Ok(())
                },
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

    fn handle_interaction(&self, event: Interaction, tree: &Component) {
        match event {
            Interaction::Pressed(point) => {
                let lc = self.layout_context.borrow();
                if let Some(node) = lc.find_node_at(point, &tree) {
                    println!("Pressed: {:?}", node);
                }
            }
            _ => {}
        }
    }
}