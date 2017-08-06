extern crate weld;
extern crate futures;
extern crate pretty_env_logger;
extern crate webrender;

use weld::application::Application;
use weld::model::*;
use weld::window::Interaction;
use weld::layout::{FlexDirection, Percent, Point, Wrap};
use weld::layout::FlexStyle::*;
use weld::layout::Align::*;
use webrender::api::*;

#[derive(Debug)]
struct Container {}

impl Renderer for Container {
    fn id(&self) -> &'static str { "Container" }
    fn render(&self, context: &mut RenderContext) {
        let bounds = context.bounds();
        context.push(RenderElement::Rect(bounds, ColorF::new(1.0, 0.0, 0.0, 1.0)));
        context.next();
    }
}

fn container() -> Component {
    Component::new(Container {})
}

#[derive(Debug)]
struct Button {}

impl Renderer for Button {
    fn id(&self) -> &'static str { "Button" }
    fn render(&self, context: &mut RenderContext) {
        let bounds = context.bounds();
        context.push(RenderElement::Rect(bounds, ColorF::new(0.0, 0.0, 1.0, 1.0)));
        context.next();
    }
}

fn button() -> Component {
    Component::new(Button {})
}

#[derive(Clone, Debug)]
struct MyAppState {
    button_width: i32,
}

impl State for MyAppState {
    fn build(&self) -> Component {
        container()
            .styles(vec![
                Width(100.percent()),
                Height(100.percent()),
                FlexDirection(FlexDirection::Row),
                Padding(25.point()),
                AlignItems(FlexStart),
                FlexWrap(Wrap::Wrap)
            ])
            .child(
                button()
                    .styles(vec![
                        Width(self.button_width.point()),
                        Height(32.point()),
                    ])
                    .name("button")
                    .on(Box::new(|state: Self, event| {
                        match *event {
                            Interaction::Pressed => {
                                println!("pressed!");
                                Ok(Self {
                                    button_width: state.button_width + 5
                                })
                            }
                            Interaction::Released => {
                                println!("released!");
                                Ok(state)
                            }
                        }
                    }))
            )
    }
}

fn main() {
    pretty_env_logger::init().unwrap();

    let app = Application::new("Demo", MyAppState {
        button_width: 100
    });

    app.run();
}