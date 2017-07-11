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

        let mut yoga = YogaNode::new();
        yoga.apply_styles(&node.data().styles);
        let visual_node = Node::new(Visual { yoga });

        let visual_parent_id = self.get_visual_node(node.parent());
        self.insert_visual_node(node, visual_node, visual_parent_id);

        for child_id in node.children() {
            self.add_node(child_id);
        }
    }

    fn get_visual_node(&self, logical_parent_node: Option<&NodeId>) -> Option<NodeId> {
        match logical_parent_node {
            Some(logical_parent_id) => {
                let key = &self.logical_tree.get(logical_parent_id).unwrap().data().id;
                Some(self.node_to_visual_map.get(key).unwrap().clone())
            }
            None => None
        }
    }

    fn insert_visual_node(&mut self, logical_node: &Node<Box<Component>>, mut visual_node: Node<Visual>, visual_parent_id: Option<NodeId>) {
        // The new node has a visual parent, so attach the YogaNode to the parent YogaNode as well
        if let Some(ref vpi) = visual_parent_id {
            let mut visual = visual_node.data_mut();
            let mut parent_yoga = &mut self.visual_tree.get_mut(&vpi).unwrap().data_mut().yoga;
            let child_count = parent_yoga.child_count();
            parent_yoga.insert_child(&mut visual.yoga, child_count);
        }

        let insert_behavior = match visual_parent_id {
            Some(ref vpi) => InsertBehavior::UnderNode(&vpi),
            None => InsertBehavior::AsRoot
        };
        let visual_node_id = Some(self.visual_tree.insert(visual_node, insert_behavior).unwrap());
        self.node_to_visual_map.insert(logical_node.data().id, visual_node_id.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use components;
    use components::panel::PanelSize;
    use component_tree::ComponentTree;
    use yoga::{Direction, Layout, Percent, FlexStyle};

    #[test]
    fn test_splitter() {
        let mut tree = ComponentTree::new();
        let root = tree.add_node(components::SplitterBuilder::new(), None);

        let mut panel1: Component = components::PanelBuilder::new().into();
        //panel1.data_mut().put(PanelSize { size: (30.percent(), 100.percent()) });
        panel1.styles = vec![
            FlexStyle::Width(30.percent()),
            FlexStyle::Height(100.percent())
        ];
        tree.add_node(panel1, Some(&root));

        let mut panel2: Component = components::PanelBuilder::new().into();
        //panel2.data_mut().put(PanelSize { size: (70.percent(), 100.percent()) });
        panel2.styles = vec![
            FlexStyle::Width(70.percent()),
            FlexStyle::Height(100.percent())
        ];
        tree.add_node(panel2, Some(&root));

        let visual_tree = VisualBuilder::new(tree.tree()).build_visual_tree();

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