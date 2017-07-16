use gleam::gl;
use glutin;
use glutin::GlContext;
use webrender;
use webrender::api::*;
use component::Component;
use events::Event;
use layout_context::LayoutContext;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;

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
    proxy: glutin::EventsLoopProxy
}

impl Notifier {
    fn new(proxy: glutin::EventsLoopProxy) -> Notifier {
        Notifier { proxy: proxy }
    }
}

impl RenderNotifier for Notifier {
    fn new_frame_ready(&mut self) {
        #[cfg(not(target_os = "android"))]
        let _ = self.proxy.wakeup().unwrap();
    }

    fn new_scroll_frame_ready(&mut self, _composite_needed: bool) {
        #[cfg(not(target_os = "android"))]
        let _ = self.proxy.wakeup().unwrap();
    }
}

pub struct WebrenderWindow {
    inner: Arc<Mutex<WebrenderWindowData>>
}

pub struct WebrenderWindowData {
    title: &'static str,
    render_context: Option<RenderContext>,
    tree_context: Option<TreeContext>
}

struct RenderContext {
    event_loop_proxy: glutin::EventsLoopProxy,
}

struct TreeContext {
    tree: Arc<Mutex<Component>>,
    epoch: Epoch,
    rendered_epoch: Epoch
}

impl WebrenderWindow {
    pub fn new(title: &'static str) -> WebrenderWindow {
        WebrenderWindow {
            inner: Arc::new(Mutex::new(WebrenderWindowData {
                title,
                render_context: None,
                tree_context: None
            }))
        }
    }

    pub fn start_thread(&mut self, event_sender: Sender<Event>) -> thread::JoinHandle<()> {
        let local_self = self.inner.clone();
        thread::spawn(move || {
            run_gl(local_self, event_sender);
        })
    }

    pub fn update(&mut self, root: Arc<Mutex<Component>>, epoch: &Epoch) {
        let mut inner = self.inner.lock().unwrap();

        // Replace the tree
        inner.tree_context = Some(TreeContext {
            tree: root,
            epoch: (*epoch).clone(),
            rendered_epoch: Epoch(<u32>::max_value())
        });

        // Notify the event loop
        if let Some(ref context) = inner.render_context {
            let _ = context.event_loop_proxy.wakeup().unwrap();
        }
    }
}

fn run_gl(local_self: Arc<Mutex<WebrenderWindowData>>, event_sender: Sender<Event>) {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title(local_self.lock().unwrap().title)
        .with_multitouch();

    let context = glutin::ContextBuilder::new().with_vsync(false);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

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

    let (mut renderer, sender) = webrender::renderer::Renderer::new(gl, opts, DeviceUintSize::new(width, height)).unwrap();

    let api = sender.create_api();
    api.set_root_pipeline(PipelineId(0, 0));

    let notifier = Box::new(Notifier::new(events_loop.create_proxy()));
    renderer.set_render_notifier(notifier);

    let mut layout_context = LayoutContext::new();

    let mut busy_rendering = AtomicBool::new(false);
    let mut need_render = true;

    let mut mouse = WorldPoint::zero();

    local_self.lock().unwrap().render_context = Some(RenderContext {
        event_loop_proxy: events_loop.create_proxy()
    });

    events_loop.run_forever(|event| {
        let size = gl_window.get_inner_size_pixels().unwrap();
        match event {
            glutin::Event::Awakened => {
                debug!("Awakened");
                renderer.update();
                renderer.render(DeviceUintSize::new(size.0, size.1));
                let _ = gl_window.swap_buffers().unwrap();
                *busy_rendering.get_mut() = false;
            }
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => return glutin::ControlFlow::Break,
                glutin::WindowEvent::Resized(w, h) => {
                    gl_window.resize(w, h);
                    need_render = true;
                },
                glutin::WindowEvent::MouseMoved { position: (x, y), .. } => {
                    mouse = WorldPoint::new(x as f32, y as f32);
                }
                glutin::WindowEvent::MouseInput { button: glutin::MouseButton::Left, state: glutin::ElementState::Pressed, .. } => {
                    if let Some(ref mut context) = local_self.lock().unwrap().tree_context {
                        let node = layout_context
                            .find_node_at(mouse, &context.tree.lock().unwrap())
                            .map(|node| *node.id());
                        let _ = event_sender.send(Event::Pressed(node));
                    }
                },
                _ => (),
            },
            _ => ()
        }

        if let Some(ref mut context) = local_self.lock().unwrap().tree_context {
            if context.rendered_epoch != context.epoch {
                need_render = true;
            }
        }

        if need_render {
            if busy_rendering.compare_and_swap(false, true, Ordering::Relaxed) == false {
                debug!("Was not busy rendering, so starting now...");
                need_render = false;

                let glsize = gl_window.get_inner_size().unwrap();
                let layout_size = LayoutSize::new(glsize.0 as f32, glsize.1 as f32);

                if let Some(ref mut context) = local_self.lock().unwrap().tree_context {
                    generate_frame(&api, &layout_size, &context.epoch, &mut layout_context, &context.tree.lock().unwrap());
                    context.rendered_epoch = context.epoch;
                }
            }
        }

        glutin::ControlFlow::Continue
    });

    let _ = event_sender.send(Event::ApplicationClosed);
}

fn generate_frame(api: &RenderApi, layout_size: &LayoutSize, epoch: &Epoch, layout_context: &mut LayoutContext, tree: &Component) {
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