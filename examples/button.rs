extern crate weld;
extern crate pretty_env_logger;

use weld::application::Application;
use weld::component::panel;
use weld::component::Configuration::*;
use weld::layout::{FlexDirection, Percent, Point, Wrap};
use weld::layout::FlexStyle::*;
use weld::layout::Align::*;

fn main() {
    pretty_env_logger::init().unwrap();

    let mut app = Application::new("Demo");

    let root = panel(vec![
        Styles(vec![
            Width(100.percent()),
            Height(100.percent()),
            FlexDirection(FlexDirection::Row),
            Padding(25.point()),
            AlignItems(FlexStart),
            FlexWrap(Wrap::Wrap)
        ]),
        //Event(Box::new(|event| { println!("Clicked the background, eh?"); })),
        Child(
            panel(vec![
                Styles(vec![
                    Width(100.point()),
                    Height(32.point()),
                ]),
                //Event(Box::new(|event| { println!("Thanks for clicking the small button"); }))
            ])
        )
    ]);

    app.run(root);
}