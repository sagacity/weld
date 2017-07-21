use component::ComponentId;
use glutin;

#[derive(Debug)]
pub enum Event {
    ApplicationClosed,
    WindowClosed,
    NotifyRenderComplete,
    Pressed(Option<ComponentId>),
    GlutinEvent(glutin::Event),
    GlutinWindowEvent(glutin::WindowEvent)
}