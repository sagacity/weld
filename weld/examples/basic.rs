extern crate weld;
extern crate weld_core;

use weld_core::window::WindowFactory;
use weld_core::components;
use weld_core::components::component::*;
use weld_core::components::panel::PanelSize;
use weld_core::component_tree::ComponentTree;

fn main() {
    let mut window = WindowFactory::new("Demo");

    window.update_tree(&|| {
        let mut tree = ComponentTree::new();
        let root = tree.add_node(components::SplitterBuilder::new(), None);

        let mut panel1: Component = components::PanelBuilder::new().into();
        panel1.data_mut().put(PanelSize { size: Size::Relative(PercentageSize::new(30.0, 100.0)) });
        tree.add_node(panel1, Some(&root));

        let mut panel2: Component = components::PanelBuilder::new().into();
        panel2.data_mut().put(PanelSize { size: Size::Relative(PercentageSize::new(70.0, 100.0)) });
        tree.add_node(panel2, Some(&root));

        tree
    });

    window.run();
}