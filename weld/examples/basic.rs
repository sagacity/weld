extern crate weld;
extern crate weld_core;

use weld_core::window::WindowFactory;
use weld_core::components;
use weld_core::component_tree::ComponentTree;

fn main() {
    let mut window = WindowFactory::new("Demo");

    window.update_tree(&|| {
        let mut tree = ComponentTree::new();
        let root = tree.add_node(components::SplitterBuilder::new(), None);
        tree.add_node(components::PanelBuilder::new(), Some(&root));
        tree.add_node(components::PanelBuilder::new(), Some(&root));
        tree
    });

    window.run();
}