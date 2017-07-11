use webrender_api::*;
use component_tree::ComponentTree;
use components::component::*;
use id_tree::{Tree, TreeBuilder, NodeId, Node, InsertBehavior};
use std::collections::HashMap;
use std::ops::Deref;
use yoga::Node as YogaNode;
use yoga::Direction;
use snowflake::ProcessUniqueId;

pub struct Theme {}

impl Theme {
    pub fn new() -> Theme {
        Theme {}
    }

    pub fn build_display_list(&self, _: &mut DisplayListBuilder, tree: &ComponentTree, size: &LayoutSize) {
        let bounds = LayoutRect::new(LayoutPoint::new(0.0, 0.0), size.clone());
        let visual_tree = VisualBuilder::new(tree.tree()).build_visual_tree();

        for visual_node in visual_tree.traverse_pre_order(visual_tree.root_node_id().unwrap()).unwrap() {
            //println!("{:?}", visual_node.data());
        }
    }
}

struct Visual {
    yoga: YogaNode
}

struct VisualBuilder<'a> {
    logical_tree: &'a Tree<Box<Component>>,
    visual_tree: Tree<Visual>,
    node_to_visual_map: HashMap<ProcessUniqueId, NodeId>
}

impl<'a> VisualBuilder<'a> {
    fn new(tree: &'a Tree<Box<Component>>) -> VisualBuilder<'a> {
        VisualBuilder {
            logical_tree: tree,
            visual_tree: TreeBuilder::new().build(),
            node_to_visual_map: HashMap::new()
        }
    }

    fn build_visual_tree(mut self) -> Tree<Visual> {
        self.add_node(self.logical_tree.root_node_id().unwrap());

        let root = self.visual_tree.root_node_id().unwrap().clone();
        self.visual_tree.get_mut(&root).unwrap().data_mut().yoga.calculate_layout(1000.0, 1000.0, Direction::LTR);

        self.visual_tree
    }

    fn add_node(&mut self, logical_node_id: &NodeId) {
        let node = self.logical_tree.get(logical_node_id).unwrap();
        println!("Adding {:?}", node.data().component_type);

        let mut visual_node = Node::new(Visual { yoga: YogaNode::new() });
        let mut visual_node_id = None;

        if let Some(parent_id) = node.parent() {
            let visual_parent_id = self.node_to_visual_map.get(&self.logical_tree.get(parent_id).unwrap().data().id).unwrap();
            {
                let mut visual = visual_node.data_mut();
                let mut parent_yoga = &mut self.visual_tree.get_mut(visual_parent_id).unwrap().data_mut().yoga;
                let child_count = parent_yoga.child_count();
                parent_yoga.insert_child(&mut visual.yoga, child_count);
            }

            visual_node_id = Some(self.visual_tree.insert(visual_node, InsertBehavior::UnderNode(visual_parent_id)).unwrap());
        } else {
            visual_node_id = Some(self.visual_tree.insert(visual_node, InsertBehavior::AsRoot).unwrap());
        }

        self.node_to_visual_map.insert(node.data().id, visual_node_id.unwrap());

        for child_id in node.children() {
            self.add_node(child_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use components;
    use components::panel::PanelSize;
    use component_tree::ComponentTree;
    use yoga::{Direction, Layout, Percent};

    #[test]
    fn test_splitter() {
        let mut tree = ComponentTree::new();
        let root = tree.add_node(components::SplitterBuilder::new(), None);

        let mut panel1: Component = components::PanelBuilder::new().into();
        panel1.data_mut().put(PanelSize { size: (30.percent(), 100.percent()) });
        tree.add_node(panel1, Some(&root));

        let mut panel2: Component = components::PanelBuilder::new().into();
        panel2.data_mut().put(PanelSize { size: (70.percent(), 100.percent()) });
        tree.add_node(panel2, Some(&root));

        let mut visual_tree = VisualBuilder::new(tree.tree()).build_visual_tree();

        let sizes: Vec<Layout> = visual_tree.traverse_pre_order(visual_tree.root_node_id().unwrap())
            .unwrap()
            .map(|node| node.data().yoga.get_layout())
            .collect();
        println!("{:?}", sizes);

        /*let bounds = LayoutRect::new(LayoutPoint::new(0.0, 0.0), LayoutSize::new(1000.0, 1000.0));
        let sizes: Vec<LayoutSize> = visual_tree.traverse_pre_order(visual_tree.root_node_id().unwrap())
            .unwrap()
            .map(|node| node.data().rect.as_layout_rect(&bounds).size)
            .collect();

        assert_eq!(sizes, vec![
            LayoutSize::new(1000.0, 1000.0), // root
            LayoutSize::new(1000.0, 1000.0),
            LayoutSize::new(300.0, 1000.0),
            LayoutSize::new(1000.0, 1000.0),
            LayoutSize::new(700.0, 1000.0),
            LayoutSize::new(1000.0, 1000.0)
        ]);*/
    }
}