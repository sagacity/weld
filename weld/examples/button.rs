extern crate weld;
extern crate weld_core;
extern crate pretty_env_logger;

use weld_core::application::Application;
use weld_core::component::panel;
use weld_core::layout::{FlexDirection, Percent, Point};
use weld_core::layout::FlexStyle::*;
use weld_core::layout::Align::*;

fn main() {
    pretty_env_logger::init().unwrap();

    let app = Application::new("Demo");

    let root = panel(vec![
        Width(100.percent()),
        Height(100.percent()),
        FlexDirection(FlexDirection::Row),
        Padding(25.point()),
        AlignItems(Center)
    ], vec![
        panel(vec![
            Width(100.point()),
            Height(32.point()),
        ], vec![]),
    ]);

    app.run(root);
}