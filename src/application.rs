use model::{Component, InvocationError, State};
use window::{WindowEvent, WebrenderWindow, RendererHandle};
use layout_context::LayoutContext;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use futures::Stream;
use tokio_core::reactor::Core;

pub struct Application<S: State> {
    title: &'static str,
    layout_context: Rc<RefCell<LayoutContext>>,
    state: S,
}

impl<S: State> Application<S> {
    pub fn new(title: &'static str, state: S) -> Self {
        let layout_context = Rc::new(RefCell::new(LayoutContext::new()));

        Application {
            title,
            layout_context,
            state,
        }
    }

    pub fn run(mut self) {
        let (mut renderer, event_stream) = WebrenderWindow::new(self.title, self.layout_context.clone());

        let new_state = self.state.clone();
        let mut tree = self.update_tree(&mut renderer, new_state);

        let event_logger = event_stream.for_each(|event| {
            //println!("event: {:?}", event);
            match event {
                WindowEvent::Interaction(_, _) => {
                    let new_tree = match self.handle_interaction(event, &tree.lock().unwrap()) {
                        Ok(new_state) => {
                            Some(self.update_tree(&mut renderer, new_state))
                        },
                        Err(_) => {
                            None
                        }
                    };

                    if let Some(t) = new_tree {
                        tree = t;
                    }

                    Ok(())
                }
                WindowEvent::NotifyRenderComplete => {
                    renderer.update();
                    Ok(())
                }
                WindowEvent::ApplicationClosed => {
                    //renderer.stop();
                    Err(())
                }
                _ => Ok(())
            }
        });

        let mut core = Core::new().unwrap();
        let _ = core.run(event_logger);
    }

    fn handle_interaction(&self, event: WindowEvent, tree: &Component) -> Result<S, InvocationError> {
        match event {
            WindowEvent::Interaction(point, interaction) => {
                let lc = self.layout_context.borrow();
                if let Some(node) = lc.find_node_at(point, &tree) {
                    println!("Interaction for: {:?}", node);
                    node.invoke(&self.state, interaction)
                } else {
                    Err(InvocationError)
                }
            }
            _ => {
                Err(InvocationError)
            }
        }
    }

    fn update_tree(&mut self, renderer: &mut RendererHandle, new_state: S) -> Arc<Mutex<Component>> {
        let tree = Arc::new(Mutex::new(new_state.build()));
        renderer.set_tree(tree.clone());
        renderer.render();
        self.state = new_state;

        tree
    }
}