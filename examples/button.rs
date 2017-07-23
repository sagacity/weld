extern crate weld;
extern crate futures;
extern crate pretty_env_logger;

use weld::application::Application;
use weld::component::*;
use weld::component::Configuration::*;
use weld::layout::{FlexDirection, Percent, Point, Wrap};
use weld::layout::FlexStyle::*;
use weld::layout::Align::*;
use weld::component::State;
use futures::stream;
use futures::stream::*;

struct ButtonApp {
    data: ButtonState
}

#[derive(Clone)]
struct ButtonState {
    counter: u32
}

impl State for ButtonApp {
    type Data = ButtonState;

    /*fn handle(&self, handler: &Fn(&ButtonState) -> BoxStream<ButtonState, ()>) -> BoxStream<Self::Data, ()> {
        handler(&self.data)
    }*/

    fn data(&self) -> &ButtonState {
        &self.data
    }

    fn build(&self, context: BuildContext) -> Component {
        let mut children = Vec::new();
        for _ in 0..self.data.counter {
            children.push(Self::panel(vec![
                Styles(vec![
                    Width(100.point()),
                    Height(32.point()),
                ]),
            ]).into());
        }

        Self::panel(vec![
            Styles(vec![
                Width(100.percent()),
                Height(100.percent()),
                FlexDirection(FlexDirection::Row),
                Padding(25.point()),
                AlignItems(FlexStart),
                FlexWrap(Wrap::Wrap)
            ]),
            Child(
                Self::panel(vec![
                    Styles(vec![
                        Width(100.point()),
                        Height(32.point()),
                    ]),
                    //Event(Box::new(|event| { println!("Thanks for clicking the small button"); }))
                ]).pressed(&|state| {
                    let mut new_state = state.clone();
                    new_state.counter = state.counter + 1;
                    stream::once::<Self::Data, ()>(Ok(new_state)).boxed()
                }).into()
            ),
            Children(children)
        ]).into()
    }
}

fn main() {
    pretty_env_logger::init().unwrap();

    let app = Application::new("Demo");

    app.run(ButtonApp {
        data: ButtonState {
            counter: 0
        }
    });
}