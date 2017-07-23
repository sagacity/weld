use glutin;
use webrender::api::WorldPoint;

#[derive(Debug)]
pub enum Event {
    ApplicationClosed,
    WindowClosed,
    NotifyRenderComplete,
    Interaction(Interaction),
    GlutinEvent(glutin::Event),
    GlutinWindowEvent(glutin::WindowEvent)
}

#[derive(Debug)]
pub enum Interaction {
    Pressed(WorldPoint),
    Released(WorldPoint)
}