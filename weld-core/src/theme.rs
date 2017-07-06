use webrender_api::*;
use component_tree::ComponentTree;
use components::component::*;
use components::panel::PanelSize;
use id_tree;
use id_tree::{Tree, TreeBuilder, NodeId};
use std::collections::HashMap;

pub struct Theme {
}

impl Theme {
    pub fn new() -> Theme {
        Theme {
        }
    }

    pub fn build_display_list(&self, builder: &mut DisplayListBuilder, tree: &ComponentTree, size: &LayoutSize) {
        let bounds = LayoutRect::new(LayoutPoint::new(0.0, 0.0), size.clone());
        let visual_tree = VisualBuilder::new(tree.tree()).traverse(bounds);

        for visual_node in visual_tree.traverse_pre_order(visual_tree.root_node_id().unwrap()).unwrap() {
            println!("{:?}", visual_node.data());
        }
    }
}

type VisualNodeId = NodeId;
type LogicalNodeId = NodeId;

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

    fn traverse(mut self, bounds: LayoutRect) -> Tree<Visual> {
        let visual_root_id = self.visual_tree.insert(id_tree::Node::new(Visual { rect: bounds }), id_tree::InsertBehavior::AsRoot).unwrap();
        self.traverse_node(&visual_root_id, self.logical_tree.root_node_id().unwrap(), bounds);
        self.visual_tree
    }

    fn traverse_node(&mut self, visual_parent_id: &VisualNodeId, node_id: &LogicalNodeId, bounds: LayoutRect) {
        use id_tree::InsertBehavior::*;

        let node = self.logical_tree.get(node_id).unwrap();
        let component = node.data();

        let mut visual_node_id = None;
        let rect = component.size.as_layout_rect(&bounds);

        match component.component_type {
            Type::Splitter => {
                // Add the main splitter visual
                let splitter_id = self.visual_tree.insert(id_tree::Node::new(Visual {
                    rect: rect
                }), UnderNode(visual_parent_id)).unwrap();
                println!("Adding splitter");

                // Add a splitter pane for every child
                for child_node_id in node.children() {
                    println!("Adding child");
                    let child_component = self.logical_tree.get(&child_node_id).unwrap().data();
                    let visual_child_node_id = self.visual_tree.insert(id_tree::Node::new(Visual {
                        rect: child_component.data().get::<PanelSize>().unwrap_or(&PanelSize { size: Size::Relative(PercentageSize::new(100.0, 100.0)) }).size.as_layout_rect(&bounds)
                    }), UnderNode(&splitter_id)).unwrap();

                    self.set_visual_parent(child_node_id, visual_child_node_id);

                    println!("Child: {:?}", child_node_id.clone());
                }

                visual_node_id = Some(splitter_id);
            },

            Type::Panel => {
                println!("Adding panel");
                println!("Parent: {:?}", node.parent().unwrap());
                let visual_parent_node_id = self.get_visual_parent(node_id); // handle case where there is no visual parent
                let panel_id = self.visual_tree.insert(id_tree::Node::new(Visual {
                    rect: rect
                }), id_tree::InsertBehavior::UnderNode(&visual_parent_node_id)).unwrap();
            }

            _ => {}
        }

        match visual_node_id {
            Some(p) => {
                for child in node.children() {
                    self.traverse_node(&p, child, rect);
                }
            },
            None => {}
        }
    }

    fn set_visual_parent(&mut self, logical_node_id: &LogicalNodeId, visual_parent_id: VisualNodeId) {
        self.visual_parent_map.insert(logical_node_id.clone(), visual_parent_id);
    }

    fn get_visual_parent(&self, logical_node_id: &LogicalNodeId) -> VisualNodeId {
        self.visual_parent_map.get(logical_node_id).unwrap().clone()
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
        let visual_tree = VisualBuilder::new(tree.tree()).traverse(bounds);

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