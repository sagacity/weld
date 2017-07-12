use webrender_api::*;
use component_tree::ComponentTree;
use components::component::*;
use id_tree::{Tree, TreeBuilder, NodeId, Node, InsertBehavior};
use std::collections::HashMap;
use yoga::Node as YogaNode;
use yoga::Direction;
use snowflake::ProcessUniqueId;

pub struct Theme {}

impl Theme {
    pub fn new() -> Theme {
        Theme {}
    }

    pub fn build_display_list(&self, builder: &mut DisplayListBuilder, tree: &ComponentTree, size: &LayoutSize) {
        let mut visual_tree = VisualBuilder::new(tree.tree()).build_visual_tree();
        visual_tree.calculate_layout(size.width, size.height);

        self.build_node(builder, &visual_tree, visual_tree.tree().root_node_id().unwrap());
    }

    fn build_node(&self, builder: &mut DisplayListBuilder, visual_tree: &VisualTree, node_id: &NodeId) {
        use rand::{random, Closed01};

        let node = visual_tree.tree().get(node_id).unwrap();
        let yoga = &node.data().yoga;

        let color = ColorF::new(random::<Closed01<f32>>().0, random::<Closed01<f32>>().0, random::<Closed01<f32>>().0, 1.0);
        let bounds = LayoutRect::new(LayoutPoint::new(yoga.get_layout().left, yoga.get_layout().top), LayoutSize::new(yoga.get_layout().width, yoga.get_layout().height));
        println!("bounds: {:?}", bounds);
        builder.push_rect(bounds, None, color);

        for child_id in node.children() {
            self.build_node(builder, visual_tree, child_id);
        }
    }
}

struct Visual {
    yoga: YogaNode
}

struct VisualTree {
    visual_tree: Tree<Visual>
}

impl VisualTree {
    pub fn tree(&self) -> &Tree<Visual> {
        &self.visual_tree
    }

    pub fn calculate_layout(&mut self, width: f32, height: f32) {
        let root = self.visual_tree.root_node_id().unwrap().clone();
        self.visual_tree.get_mut(&root).unwrap().data_mut().yoga.calculate_layout(width, height, Direction::LTR);
    }
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

    fn build_visual_tree(mut self) -> VisualTree {
        self.add_node(self.logical_tree.root_node_id().unwrap());

        VisualTree {
            visual_tree: self.visual_tree
        }
    }

    fn add_node(&mut self, logical_node_id: &NodeId) {
        let node = self.logical_tree.get(logical_node_id).unwrap();
        println!("Adding {:?}", node.data().component_type);
        println!("Styles {:?}", node.data().styles);

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

    fn insert_visual_node(&mut self, logical_node: &Node<Box<Component>>, mut visual_node: Node<Visual>, visual_parent_id: Option<NodeId>) -> NodeId {
        // The new node has a visual parent, so attach the YogaNode to the parent YogaNode as well
        if let Some(ref vpi) = visual_parent_id {
            let mut visual = visual_node.data_mut();
            let mut parent_yoga = &mut self.visual_tree.get_mut(&vpi).unwrap().data_mut().yoga;
            let child_count = parent_yoga.child_count();
            parent_yoga.insert_child(&mut visual.yoga, child_count);
            println!("Inserted into parent yoga at pos {:?}", child_count);
        }

        let insert_behavior = match visual_parent_id {
            Some(ref vpi) => InsertBehavior::UnderNode(&vpi),
            None => InsertBehavior::AsRoot
        };
        let visual_node_id = self.visual_tree.insert(visual_node, insert_behavior).unwrap();
        self.node_to_visual_map.insert(logical_node.data().id, visual_node_id.clone());

        visual_node_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use components;
    use components::panel::PanelSize;
    use component_tree::ComponentTree;
    use yoga::{Direction, Layout, Percent, Point, FlexStyle, FlexDirection};

    #[test]
    fn test_splitter() {
        let mut tree = ComponentTree::new();
        let mut splitter: Component = components::SplitterBuilder::new(vec![
            FlexStyle::Width(1000.point()),
            FlexStyle::Height(1000.point()),
            FlexStyle::FlexDirection(FlexDirection::Row)
        ]).into();
        let root = tree.add_node(splitter, None);

        let mut panel1: Component = components::PanelBuilder::new(vec![
            FlexStyle::Width(30.percent()),
            FlexStyle::Height(100.percent())
        ]).into();
        tree.add_node(panel1, Some(&root));

        let mut panel2: Component = components::PanelBuilder::new(vec![
            FlexStyle::Width(70.percent()),
            FlexStyle::Height(100.percent())
        ]).into();
        tree.add_node(panel2, Some(&root));

        let mut visual_tree = VisualBuilder::new(tree.tree()).build_visual_tree();
        visual_tree.calculate_layout(1000.0, 1000.0);

        let sizes: Vec<Layout> = visual_tree.tree().traverse_pre_order(visual_tree.tree().root_node_id().unwrap())
            .unwrap()
            .map(|node| node.data().yoga.get_layout())
            .collect();

        assert_eq!(sizes, vec![
            Layout { left:   0.0, right:    0.0, top: 0.0, bottom: 0.0, width: 1000.0, height: 1000.0 },
            Layout { left:   0.0, right:    0.0, top: 0.0, bottom: 0.0, width:  300.0, height: 1000.0 },
            Layout { left: 300.0, right:    0.0, top: 0.0, bottom: 0.0, width:  700.0, height: 1000.0 },
        ]);
    }
}