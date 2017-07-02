extern crate weld_core;

use weld_core::component_tree::ComponentTree;
use weld_core::components::label::{Label, LabelBuilder};

#[test]
fn test_tree() {
    let mut tree = ComponentTree::new();

    let root = tree.add_node(LabelBuilder::new().caption("Parent"), None);
    let child = tree.add_node(LabelBuilder::new().caption("Child"), Some(&root));
    let child2 = tree.add_node(LabelBuilder::new().caption("Child2"), Some(&root));

    assert_eq!((tree.get(&root) as &Label).get_caption().unwrap().caption,
    "Parent");
}