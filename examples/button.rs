extern crate weld;
extern crate futures;
extern crate pretty_env_logger;

use weld::application::Application;
use weld::model::*;
use weld::layout::{FlexDirection, Percent, Point, Wrap};
use weld::layout::FlexStyle::*;
use weld::layout::Align::*;

#[derive(Debug)]
struct Container {}

impl Renderer for Container {
    fn id(&self) -> &'static str { "Container" }
    fn render(&self, context: &mut RenderContext) {
        unimplemented!()
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
        unimplemented!()
    }
}

fn button() -> Component {
    Component::new(Button {})
}

#[derive(Clone, Debug)]
struct MyAppState {}

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
                        Width(100.point()),
                        Height(32.point()),
                    ])
                    .name("button")
            )
    }
}

fn main() {
    pretty_env_logger::init().unwrap();

    let app = Application::new("Demo");

    app.run(MyAppState {});
}