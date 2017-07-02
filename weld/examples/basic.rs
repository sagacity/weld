extern crate weld;
extern crate weld_core;

use weld::window::WindowFactory;
use weld_core::components::label::LabelBuilder;
use weld_core::components::panel::PanelBuilder;
use weld_core::component_tree::ComponentTree;

fn main() {
    let mut window = WindowFactory::new("Demo");

    window.update_tree(&|| {
        let mut tree = ComponentTree::new();
        let root = tree.add_node(PanelBuilder::new(), None);
        tree.add_node(LabelBuilder::new().caption("Child"), Some(&root));
        tree
    });

    window.run();
}