extern crate weld;
extern crate weld_core;
extern crate pretty_env_logger;

use weld_core::application::Application;
use weld_core::component::panel2;
use weld_core::layout::{FlexDirection, Percent, Point, Wrap};
use weld_core::layout::FlexStyle::*;
use weld_core::layout::Align::*;

fn main() {
    pretty_env_logger::init().unwrap();

    let mut app = Application::new("Demo");

    let root = panel2(vec![
        Width(100.percent()),
        Height(100.percent()),
        FlexDirection(FlexDirection::Row),
        Padding(25.point()),
        AlignItems(FlexStart),
        FlexWrap(Wrap::Wrap)
    ], vec![
        panel2(vec![
            Width(100.point()),
            Height(32.point()),
        ], vec![], |event| { println!("Thanks for clicking the small button"); }),
    ], |event| { println!("Clicked the background, eh?"); });

    app.run(root);
}