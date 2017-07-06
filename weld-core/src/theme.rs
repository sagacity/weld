use webrender_api::*;
use component_tree::ComponentTree;
use components::component::*;
use components::panel::PanelSize;
use id_tree::{Tree, TreeBuilder, NodeId, Node, InsertBehavior};
use std::collections::HashMap;
use std::ops::Deref;

pub struct Theme {
}

impl Theme {
    pub fn new() -> Theme {
        Theme {
        }
    }

    pub fn build_display_list(&self, _: &mut DisplayListBuilder, tree: &ComponentTree, size: &LayoutSize) {
        let bounds = LayoutRect::new(LayoutPoint::new(0.0, 0.0), size.clone());
        let visual_tree = VisualBuilder::new(tree.tree()).build_visual_tree(bounds);

        for visual_node in visual_tree.traverse_pre_order(visual_tree.root_node_id().unwrap()).unwrap() {
            println!("{:?}", visual_node.data());
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct VisualNodeId { id: NodeId }

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct LogicalNodeId { id: NodeId }

impl Deref for VisualNodeId {
    type Target = NodeId;

    fn deref(&self) -> &NodeId {
        &self.id
    }
}

impl Deref for LogicalNodeId {
    type Target = NodeId;

    fn deref(&self) -> &NodeId {
        &self.id
    }
}

#[derive(Debug)]
struct Visual {
    rect: LayoutRect,
}

struct VisualBuilder<'a> {
    logical_tree: &'a Tree<Box<Component>>,
    visual_tree: Tree<Visual>,
    visual_parent_map: HashMap<LogicalNodeId, VisualNodeId>,
}

impl<'a> VisualBuilder<'a> {
    fn new(tree: &'a Tree<Box<Component>>) -> VisualBuilder<'a> {
        VisualBuilder {
            logical_tree: tree,
            visual_tree: TreeBuilder::new().build(),
            visual_parent_map: HashMap::new(),
        }
    }

    fn build_visual_tree(mut self, bounds: LayoutRect) -> Tree<Visual> {
        let visual_root_id = VisualNodeId { id: self.visual_tree.insert(Node::new(Visual { rect: bounds }), InsertBehavior::AsRoot).unwrap() };
        let logical_root_id = LogicalNodeId { id: self.logical_tree.root_node_id().unwrap().clone() };
        self.set_visual_parent(&logical_root_id, &visual_root_id);

        self.traverse_node(&logical_root_id, bounds);

        self.visual_tree
    }

    fn traverse_node(&mut self, node_id: &LogicalNodeId, bounds: LayoutRect) {
        let node = self.logical_tree.get(node_id).unwrap();
        let component = node.data();

        let rect = component.size.as_layout_rect(&bounds);

        // The current logical node will already have a visual parent registered, so find it and use it as the tree inserter
        let visual_parent_id;
        let visual_parent_inserter = match self.get_visual_parent(node_id) {
            Some(vpi) => {
                visual_parent_id = vpi.clone();
                InsertBehavior::UnderNode(&visual_parent_id)
            },
            None => panic!("There is no visual parent available for node {:?}", node_id)
        };

        // Insert the main visual node
        let visual_node_id = self.visual_tree.insert(Node::new(Visual {
            rect: rect
        }), visual_parent_inserter).unwrap();

        // Insert additional nodes, if needed
        match component.component_type {
            Type::Splitter => {
                // Add a splitter pane for every child
                for child_node_id in node.children() {
                    let child_component = self.logical_tree.get(&child_node_id).unwrap().data();
                    let visual_child_node_id = VisualNodeId {
                        id: self.visual_tree.insert(Node::new(Visual {
                            rect: child_component.data().get::<PanelSize>().unwrap_or(&PanelSize { size: Size::Relative(PercentageSize::new(100.0, 100.0)) }).size.as_layout_rect(&bounds)
                        }), InsertBehavior::UnderNode(&visual_node_id)).unwrap()
                    };

                    self.set_visual_parent(&LogicalNodeId { id: child_node_id.clone() }, &visual_child_node_id);
                }
            },

            _ => {}
        }

        for child_node_id in node.children() {
            self.traverse_node(&LogicalNodeId { id: child_node_id.clone() }, rect);
        }
    }

    fn set_visual_parent(&mut self, logical_node_id: &LogicalNodeId, visual_parent_id: &VisualNodeId) {
        self.visual_parent_map.insert(logical_node_id.clone(), visual_parent_id.clone());
    }

    fn get_visual_parent(&self, logical_node_id: &LogicalNodeId) -> Option<&VisualNodeId> {
        self.visual_parent_map.get(logical_node_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use components;
    use components::panel::PanelSize;
    use component_tree::ComponentTree;

    #[test]
    fn test_splitter() {
        let mut tree = ComponentTree::new();
        let root = tree.add_node(components::SplitterBuilder::new(), None);

        let mut panel1: Component = components::PanelBuilder::new().into();
        panel1.data_mut().put(PanelSize { size: Size::Relative(PercentageSize::new(30.0, 100.0)) });
        tree.add_node(panel1, Some(&root));

        let mut panel2: Component = components::PanelBuilder::new().into();
        panel2.data_mut().put(PanelSize { size: Size::Relative(PercentageSize::new(70.0, 100.0)) });
        tree.add_node(panel2, Some(&root));

        let bounds = LayoutRect::new(LayoutPoint::new(0.0, 0.0), LayoutSize::new(1000.0, 1000.0));
        let visual_tree = VisualBuilder::new(tree.tree()).build_visual_tree(bounds);

        let sizes: Vec<LayoutSize> = visual_tree.traverse_pre_order(visual_tree.root_node_id().unwrap())
            .unwrap()
            .map(|node| node.data().rect.size)
            .collect();

        assert_eq!(sizes, vec![
            LayoutSize::new(1000.0, 1000.0), // root
            LayoutSize::new(1000.0, 1000.0),
            LayoutSize::new(300.0, 1000.0),
            LayoutSize::new(1000.0, 1000.0),
            LayoutSize::new(700.0, 1000.0),
            LayoutSize::new(1000.0, 1000.0)
        ]);
    }
}