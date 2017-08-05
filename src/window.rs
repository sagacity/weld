use gleam::gl;
use glutin;
use glutin::GlContext;
use webrender;
use webrender::api::*;
use layout_context::LayoutContext;
use futures::{Async, Poll, Stream};
use futures::task;
use model::Component;
use events;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
pub struct Epoch(pub u32);

impl Epoch {
    pub fn next(&mut self) -> Self {
        let val = self.0;
        self.0 = self.0 + 1;
        Epoch(val)
    }
}

struct Notifier {
    window_events_tx: mpsc::Sender<events::Event>
}

impl RenderNotifier for Notifier {
    fn new_frame_ready(&mut self) {
        info!("new_frame_ready");
        #[cfg(not(target_os = "android"))]
        self.window_events_tx.send(events::Event::NotifyRenderComplete).unwrap();
    }

    fn new_scroll_frame_ready(&mut self, _composite_needed: bool) {
        info!("new_scroll_frame_ready");
        #[cfg(not(target_os = "android"))]
        self.window_events_tx.send(events::Event::NotifyRenderComplete).unwrap();
    }
}

pub struct RendererHandle {
    epoch: Epoch,
    layout_context: Rc<RefCell<LayoutContext>>,
    tree: Option<Arc<Mutex<Component>>>,
    window_size: (u32, u32),
    gl_window: glutin::GlWindow,
    renderer: webrender::renderer::Renderer,
    api: webrender::api::RenderApi,
}

impl RendererHandle {
    pub fn update(&mut self) {
        self.renderer.update();
        self.renderer.render(DeviceUintSize::new(self.window_size.0, self.window_size.1));
        self.gl_window.swap_buffers().unwrap();
    }

    pub fn render(&mut self) {
        if let Some(ref tree) = self.tree {
            info!("render()");
            let layout_size = LayoutSize::new(self.window_size.0 as f32, self.window_size.1 as f32);

            generate_frame(&self.api, &layout_size, &self.epoch.next(), &mut self.layout_context.borrow_mut(), &tree.lock().unwrap());
            //context.rendered_epoch = context.epoch;
        }
    }

    pub fn set_tree(&mut self, tree: Arc<Mutex<Component>>) {
        self.tree = Some(tree);
    }
}

pub struct WebrenderWindow;

impl WebrenderWindow {
    pub fn new(title: &'static str, layout_context: Rc<RefCell<LayoutContext>>) -> (RendererHandle, EventStream) {
        let (window_events_tx, window_events_rx) = mpsc::channel::<events::Event>();

        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_multitouch();

        let glutin_events = glutin::EventsLoop::new();

        let context = glutin::ContextBuilder::new().with_vsync(false);
        let gl_window = glutin::GlWindow::new(window, context, &glutin_events).unwrap();

        unsafe {
            let _ = gl_window.make_current().unwrap();
        };

        let gl = match gl::GlType::default() {
            gl::GlType::Gl => unsafe {
                gl::GlFns::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _)
            },
            gl::GlType::Gles => unsafe {
                gl::GlesFns::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _)
            }
        };

        //println!("OpenGL version {}", gl.get_string(gl::VERSION));
        //println!("Shader resource path: {:?}", res_path);

        let (width, height) = gl_window.get_inner_size_pixels().unwrap();

        let opts = webrender::RendererOptions {
            //resource_override_path: res_path,
            debug: true,
            precache_shaders: true,
            device_pixel_ratio: 1.0,
            //window.hidpi_factor(),
            ..webrender::RendererOptions::default()
        };

        let (renderer, sender) = webrender::renderer::Renderer::new(gl, opts, DeviceUintSize::new(width, height)).unwrap();

        let api = sender.create_api();
        api.set_root_pipeline(PipelineId(0, 0));

        let notifier = Box::new(Notifier {
            window_events_tx: window_events_tx.clone()
        });
        renderer.set_render_notifier(notifier);

        (RendererHandle {
            epoch: Epoch(0),
            layout_context,
            tree: None,
            window_size: (width, height),
            gl_window,
            renderer,
            api,
        }, EventStream {
            glutin_events,
            window_events: window_events_rx,
            events: VecDeque::new(),
            mouse: WorldPoint::zero()
        })
    }
}

pub struct EventStream {
    glutin_events: glutin::EventsLoop,
    window_events: mpsc::Receiver<events::Event>,
    events: VecDeque<events::Event>,
    mouse: WorldPoint,
}

impl Stream for EventStream {
    type Item = events::Event;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut polled_events = Vec::new();

        // Grab all Glutin events
        let mut mouse: WorldPoint = self.mouse;
        self.glutin_events.poll_events(|event| {
            let weld_event = match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => events::Event::WindowClosed,
                    glutin::WindowEvent::MouseMoved { position: (x, y), .. } => {
                        mouse = WorldPoint::new(x as f32, y as f32);
                        events::Event::GlutinWindowEvent(event)
                    },
                    glutin::WindowEvent::MouseInput { button: glutin::MouseButton::Left, state: glutin::ElementState::Pressed, .. } => {
                        events::Event::Interaction(events::Interaction::Pressed(mouse))
                    },
                    glutin::WindowEvent::MouseInput { button: glutin::MouseButton::Left, state: glutin::ElementState::Released, .. } => {
                        events::Event::Interaction(events::Interaction::Released(mouse))
                    },
                    _ => events::Event::GlutinWindowEvent(event)
                },
                _ => events::Event::GlutinEvent(event)
            };

            polled_events.push(weld_event);
        });
        self.mouse = mouse;
        self.events.extend(polled_events);

        // Grab all events sent by notifier
        loop {
            match self.window_events.try_recv() {
                Ok(event) => self.events.push_back(event),
                Err(_) => break
            }
        }

        // Publish in stream
        match self.events.pop_front() {
            Some(event) => {
                match event {
                    events::Event::WindowClosed => Ok(Async::Ready(None)),
                    _ => Ok(Async::Ready(Some(event)))
                }
            },
            None => {
                // No messages were polled, notify the task so we will be re-polled in a little while
                let t = task::current();
                t.notify();

                Ok(Async::NotReady)
            }
        }
    }
}

fn generate_frame(api: &RenderApi, layout_size: &LayoutSize, epoch: &Epoch, layout_context: &mut LayoutContext, tree: &Component) {
    info!("generate_frame, epoch: {}", epoch.0);
    let device_size = DeviceUintSize::new(layout_size.width as u32, layout_size.height as u32);
    let root_background_color = ColorF::new(0.0, 0.7, 0.0, 1.0);
    api.set_window_parameters(device_size, DeviceUintRect::new(DeviceUintPoint::zero(), device_size));
    api.set_display_list(Some(root_background_color),
                         webrender::api::Epoch(epoch.0),
                         *layout_size,
                         build_display_list(&layout_size, layout_context, tree).finalize(),
                         true);
    api.generate_frame(None);
}

fn build_display_list(layout_size: &LayoutSize, layout_context: &mut LayoutContext, tree: &Component) -> DisplayListBuilder {
    let mut builder = DisplayListBuilder::new(PipelineId(0, 0), *layout_size);

    layout_context.update_layout(&tree, layout_size);
    layout_context.build_display_list(&mut builder, &tree);

    builder
}