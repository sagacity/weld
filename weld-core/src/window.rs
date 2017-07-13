use gleam::gl;
use glutin;
use glutin::GlContext;
use webrender;
use webrender::api::*;
use component_tree::ComponentTree;
use theme::Theme;
use std::sync::atomic::{AtomicBool, Ordering};

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

pub trait Window {
    fn run(&mut self);
    fn update_tree(&mut self, tree_builder: &Fn() -> ComponentTree);
}

pub struct WindowFactory;

struct WebrenderWindow {
    title: &'static str,
    tree: ComponentTree
}

struct TreeBuilderContext<'a> {
    size: DeviceUintSize,
    theme: &'a Theme,
    api: &'a RenderApi,
    epoch: &'a Epoch,
}


impl WindowFactory {
    pub fn new(title: &'static str) -> Box<Window> {
        return
            Box::new(WebrenderWindow {
                title,
                tree: ComponentTree::new()
            });
    }
}

impl Window for WebrenderWindow {
    fn run(&mut self) {
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

        let theme = Theme::new();
        let mut epoch = Epoch(0);
        let mut dirty = true;
        let mut busy_rendering = AtomicBool::new(false);

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
                    }
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
                        theme: &theme,
                        api: &api,
                        epoch: &epoch
                    });
                    epoch.0 = epoch.0 + 1;
                }
            }

            glutin::ControlFlow::Continue
        });
    }

    fn update_tree(&mut self, tree_builder: &Fn() -> ComponentTree) {
        self.tree = tree_builder();
    }
}

impl WebrenderWindow {
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