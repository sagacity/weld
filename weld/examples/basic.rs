extern crate weld;
extern crate weld_core;
extern crate yoga;

use weld_core::window::WindowFactory;
use weld_core::components::*;
use weld_core::component_tree::ComponentTree;
use yoga::{FlexDirection, Percent, Point};
use yoga::FlexStyle::*;

fn main() {
    let mut window = WindowFactory::new("Demo");
    let width = window.size().width as f32;
    let height = window.size().height as f32;

    window.update_tree(&|| {
        let mut tree = ComponentTree::new();

        let root = tree.add_node(SplitterBuilder::new(vec![
            Width(width.point()),
            Height(height.point()),
            FlexDirection(FlexDirection::Row),
            Padding(25.point())
        ]), None);

        tree.add_node(PanelBuilder::new(vec![
            Width(30.percent()),
            Height(100.percent()),
        ]), Some(&root));

        tree.add_node(PanelBuilder::new(vec![
            Flex(1.0),
            Margin(20.point())
        ]), Some(&root));

        tree
    });

    window.run();
}