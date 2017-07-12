extern crate weld;
extern crate weld_core;
extern crate yoga;

use weld_core::window::WindowFactory;
use weld_core::components;
use weld_core::components::component::*;
use weld_core::components::panel::PanelSize;
use weld_core::component_tree::ComponentTree;
use yoga::{FlexStyle, FlexDirection, Percent, Point};

fn main() {
    let mut window = WindowFactory::new("Demo");
    let width = window.size().width as f32;
    let height = window.size().height as f32;

    window.update_tree(&|| {
        let mut tree = ComponentTree::new();

        let mut splitter: Component = components::SplitterBuilder::new(vec![
            FlexStyle::Width(width.point()),
            FlexStyle::Height(height.point()),
            FlexStyle::FlexDirection(FlexDirection::Row)
        ]).into();
        let root = tree.add_node(splitter, None);

        let mut panel1: Component = components::PanelBuilder::new(vec![
            FlexStyle::Width(30.percent()),
            FlexStyle::Height(100.percent())
        ]).into();
        tree.add_node(panel1, Some(&root));

        let mut panel2: Component = components::PanelBuilder::new(vec![
            FlexStyle::Width(70.percent()),
            FlexStyle::Height(100.percent())
        ]).into();
        tree.add_node(panel2, Some(&root));

        tree
    });

    window.run();
}