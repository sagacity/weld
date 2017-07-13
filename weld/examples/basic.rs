extern crate weld;
extern crate weld_core;

use weld_core::window::WindowFactory;
use weld_core::components::*;
use weld_core::component_tree::ComponentTree;
use weld_core::layout::{FlexDirection, Percent, Point};
use weld_core::layout::FlexStyle::*;

fn main() {
    let mut window = WindowFactory::new("Demo");

    window.update_tree(&|| {
        let mut tree = ComponentTree::new();

        let root = tree.add_node(SplitterBuilder::new(vec![
            Width(100.percent()),
            Height(100.percent()),
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