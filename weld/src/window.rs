use gleam::gl;
use glutin;
use webrender;
use webrender_traits::*;
use weld_core::component_tree::ComponentTree;

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
    fn run(&mut self);
}

pub struct WindowFactory;

struct WebrenderWindow {
    window: glutin::Window,
    renderer: webrender::Renderer,
    api: RenderApi,
    size: DeviceUintSize, 
    tree: ComponentTree,
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

        println!("OpenGL version {}", gl.get_string(gl::VERSION));
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

        let notifier = Box::new(Notifier::new(window.create_window_proxy()));
        renderer.set_render_notifier(notifier);

        let epoch = Epoch(0);
        let root_background_color = ColorF::new(0.3, 0.0, 0.0, 1.0);

        let pipeline_id = PipelineId(0, 0);
        let layout_size = LayoutSize::new(width as f32, height as f32);
        let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);

        let stacking_bounds = LayoutRect::new(LayoutPoint::new(0.0, 0.0),
                                              LayoutSize::new(250.0, 250.0));
        builder.push_stacking_context(ScrollPolicy::Scrollable,
                                      stacking_bounds,
                                      None,
                                      TransformStyle::Flat,
                                      None,
                                      MixBlendMode::Normal,
                                      Vec::new());

        let bounds = LayoutRect::new(LayoutPoint::new(50.0, 50.0), LayoutSize::new(100.0, 100.0));
        builder.push_rect(bounds, bounds, ColorF::new(1.0, 1.0, 1.0, 1.0));
        builder.push_box_shadow(bounds,
                                stacking_bounds,
                                bounds,
                                LayoutVector2D::new(5.0, 5.0),
                                ColorF::new(0.5, 0.5, 0.5, 0.5),
                                2.0,
                                2.0,
                                2.0,
                                BoxShadowClipMode::None);
        builder.pop_stacking_context();

        api.set_display_list(Some(root_background_color),
                             epoch,
                             LayoutSize::new(width as f32, height as f32),
                             builder.finalize(),
                             true);
        api.set_root_pipeline(pipeline_id);
        api.generate_frame(None);

        return Box::new(WebrenderWindow {
            window: window,
            renderer: renderer,
            api: api,
            size: size, 
            tree: ComponentTree::new(),
        });
    }
}

impl Window for WebrenderWindow {
    fn run(&mut self) {
        'outer: for event in self.window.wait_events() {
            let mut events = Vec::new();
            events.push(event);

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
}