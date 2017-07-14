use gleam::gl;
use glutin;
use glutin::GlContext;
use webrender;
use webrender::api::*;
use component_tree::ComponentTree;
use theme::Theme;
use events::Event;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;

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
    tree: ComponentTree
}

struct TreeBuilderContext<'a> {
    size: DeviceUintSize,
    theme: &'a mut Theme,
    api: &'a RenderApi,
    epoch: &'a Epoch,
}

impl WebrenderWindow {
    pub fn new(title: &'static str) -> WebrenderWindow {
        WebrenderWindow {
            inner: Arc::new(Mutex::new(WebrenderWindowData {
                title,
                tree: ComponentTree::new()
            }))
        }
    }

    pub fn start_thread(&self, event_sender: Sender<Event>) -> thread::JoinHandle<()> {
        let local_self = self.inner.clone();
        thread::spawn(move || {
            local_self.lock().unwrap().run_gl(event_sender);
        })
    }

    pub fn update_tree(&self, tree_builder: &Fn() -> ComponentTree) {
        self.inner.lock().unwrap().tree = tree_builder();
    }
}

impl WebrenderWindowData {
    fn run_gl(&mut self, event_sender: Sender<Event>) {
        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(self.title)
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

        let mut theme = Theme::new();
        let mut epoch = Epoch(0);
        let mut dirty = true;
        let mut busy_rendering = AtomicBool::new(false);

        let mut mouse = WorldPoint::zero();

        events_loop.run_forever(|event| {
            let size = gl_window.get_inner_size_pixels().unwrap();
            match event {
                glutin::Event::Awakened => {
                    println!("Awakened");
                    renderer.update();
                    renderer.render(DeviceUintSize::new(size.0, size.1));
                    let _ = gl_window.swap_buffers().unwrap();
                    *busy_rendering.get_mut() = false;
                }
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => return glutin::ControlFlow::Break,
                    glutin::WindowEvent::Resized(w, h) => {
                        gl_window.resize(w, h);
                        dirty = true;
                    },
                    glutin::WindowEvent::MouseMoved { position: (x, y), .. } => {
                        mouse = WorldPoint::new(x as f32, y as f32);
                    }
                    glutin::WindowEvent::MouseInput { button: glutin::MouseButton::Left, .. } => {
                        theme.find_visual_at(mouse);
                    },
                    _ => (),
                },
                _ => ()
            }

            if dirty {
                if busy_rendering.compare_and_swap(false, true, Ordering::Relaxed) == false {
                    println!("Was not busy rendering, so starting now...");
                    dirty = false;

                    self.build_tree(TreeBuilderContext {
                        size: DeviceUintSize::new(size.0, size.1),
                        theme: &mut theme,
                        api: &api,
                        epoch: &epoch
                    });
                    epoch.0 = epoch.0 + 1;
                }
            }

            glutin::ControlFlow::Continue
        });

        let _ = event_sender.send(Event::ApplicationClosed);
    }

    fn build_tree(&mut self, context: TreeBuilderContext) {
        let layout_size = LayoutSize::new(context.size.width as f32, context.size.height as f32);
        let mut builder = DisplayListBuilder::new(PipelineId(0, 0), layout_size);
        context.theme.build_display_list(&mut builder, &self.tree, &layout_size);

        let root_background_color = ColorF::new(0.0, 0.7, 0.0, 1.0);
        context.api.set_window_parameters(context.size, DeviceUintRect::new(DeviceUintPoint::zero(), context.size));
        context.api.set_display_list(Some(root_background_color),
                                     *context.epoch,
                                     layout_size,
                                     builder.finalize(),
                                     true);
        context.api.generate_frame(None);
    }
}