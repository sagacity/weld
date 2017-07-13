use gleam::gl;
use glutin;
use webrender;
use webrender::api::*;
use component_tree::ComponentTree;
use theme::Theme;

struct Notifier {
    window_proxy: glutin::WindowProxy,
}

impl Notifier {
    fn new(window_proxy: glutin::WindowProxy) -> Notifier {
        Notifier { window_proxy: window_proxy }
    }
}

impl RenderNotifier for Notifier {
    fn new_frame_ready(&mut self) {
        #[cfg(not(target_os = "android"))]
        self.window_proxy.wakeup_event_loop();
    }

    fn new_scroll_frame_ready(&mut self, _composite_needed: bool) {
        #[cfg(not(target_os = "android"))]
        self.window_proxy.wakeup_event_loop();
    }
}

pub trait Window {
    fn size(&self) -> &DeviceUintSize;
    fn run(&mut self);
    fn update_tree(&mut self, tree_builder: &Fn() -> ComponentTree);
}

pub struct WindowFactory;

struct WebrenderWindow {
    window: glutin::Window,
    renderer: webrender::Renderer,
    api: RenderApi,
    size: DeviceUintSize, 
    tree: ComponentTree,
    theme: Theme,
    epoch: u32
}

impl WindowFactory {
    pub fn new(title: &str) -> Box<Window> {
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_multitouch()
            .with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (3, 0),
            })
            .build()
            .unwrap();

        unsafe {
            window.make_current().ok();
        }

        let gl = match gl::GlType::default() {
            gl::GlType::Gl => unsafe {
                gl::GlFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
            },
            gl::GlType::Gles => unsafe {
                gl::GlesFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
            },
        };

        //println!("OpenGL version {}", gl.get_string(gl::VERSION));
        //println!("Shader resource path: {:?}", res_path);

        let (width, height) = window.get_inner_size_pixels().unwrap();

        let opts = webrender::RendererOptions {
            //resource_override_path: res_path,
            debug: true,
            precache_shaders: true,
            device_pixel_ratio: window.hidpi_factor(),
            ..webrender::RendererOptions::default()
        };

        let size = DeviceUintSize::new(width, height);
        let (renderer, sender) = webrender::renderer::Renderer::new(gl, opts, size).unwrap();
        let api = sender.create_api();

        api.set_root_pipeline(PipelineId(0, 0));

        let notifier = Box::new(Notifier::new(window.create_window_proxy()));
        renderer.set_render_notifier(notifier);

        return Box::new(WebrenderWindow {
            window: window,
            renderer: renderer,
            api: api,
            size: size, 
            tree: ComponentTree::new(),
            theme: Theme::new(),
            epoch: 0
        });
    }
}

impl Window for WebrenderWindow {
    fn size(&self) -> &DeviceUintSize {
        &self.size
    }

    fn run(&mut self) {
        'outer: loop {
            let mut events: Vec<glutin::Event> = Vec::new();
            events.push(self.window.wait_events().next().unwrap());

            for event in self.window.poll_events() {
                events.push(event);
            }

            for event in events {
                match event {
                    glutin::Event::Closed |
                    glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                    glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Q)) => {
                        break 'outer
                    }

                    glutin::Event::Resized(width, height) => {
                        self.size = DeviceUintSize::new(width, height);
                        self.build_tree();
                    }

                    glutin::Event::KeyboardInput(glutin::ElementState::Pressed,
                                                 _,
                                                 Some(glutin::VirtualKeyCode::P)) => {
                        let enable_profiler = !self.renderer.get_profiler_enabled();
                        self.renderer.set_profiler_enabled(enable_profiler);
                        self.api.generate_frame(None);
                    }

                    _ => {}//event_handler(&event, &api),
                }
            }

            self.renderer.update();
            self.renderer.render(self.size);
            self.window.swap_buffers().ok();
        }
    }

    fn update_tree(&mut self, tree_builder: &Fn() -> ComponentTree) {
        self.tree = tree_builder();
        self.build_tree();
    }
}

impl WebrenderWindow {
    fn build_tree(&mut self) {
        let layout_size = LayoutSize::new(self.size.width as f32, self.size.height as f32);
        let mut builder = DisplayListBuilder::new(PipelineId(0, 0), layout_size);
        self.theme.build_display_list(&mut builder, &self.tree, &layout_size);

        let epoch = Epoch(self.epoch);
        self.epoch = self.epoch + 1;
        let root_background_color = ColorF::new(0.0, 0.7, 0.0, 1.0);
        self.api.set_window_parameters(self.size, DeviceUintRect::new(DeviceUintPoint::new(0, 0), self.size));
        self.api.set_display_list(Some(root_background_color),
                             epoch,
                             layout_size,
                             builder.finalize(),
                             true);

        self.api.generate_frame(None);
    }
}