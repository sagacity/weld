use webrender::api::*;
use component::{Component, ComponentId};
use std::collections::HashMap;
use std::cell::{Ref, RefMut, RefCell};
use layout;

pub struct Theme {
    layout_nodes: HashMap<ComponentId, RefCell<layout::Node>>
}

impl Theme {
    pub fn new() -> Theme {
        Theme {
            layout_nodes: HashMap::new()
        }
    }

    pub fn get_layout(&mut self, node: &Component) -> layout::Layout {
        self.get_layout_node(node).get_layout()
    }

    pub fn update_layout(&mut self, root: &Component, size: &LayoutSize) {
        // HACK: Throw away old layout_nodes first
        self.layout_nodes = HashMap::new();

        self.update_layout_recursive(root);
        self.get_layout_node_mut(root).calculate_layout(size.width, size.height, layout::Direction::LTR);
    }

    fn update_layout_recursive(&mut self, node: &Component) {
        {
            let layout_node = self.layout_nodes.entry(*node.id()).or_insert_with(|| RefCell::new(layout::Node::new()));
            layout_node.borrow_mut().apply_styles(node.styles());
        }

        for child in node.children() {
            self.update_layout_recursive(child);

            let mut layout_node = self.get_layout_node_mut(node);
            let mut layout_child = self.get_layout_node_mut(child);
            let child_count = layout_node.child_count();
            layout_node.insert_child(&mut layout_child, child_count);
        }
    }

    pub fn build_display_list(&self, builder: &mut DisplayListBuilder, root: &Component) {
        self.build_display_list_recursive(builder, root);
    }

    fn build_display_list_recursive(&self, builder: &mut DisplayListBuilder, node: &Component) {
        use rand::{random, Closed01};

        let layout_node = self.get_layout_node(node);
        let layout = layout_node.get_layout();

        let color = ColorF::new(random::<Closed01<f32>>().0, random::<Closed01<f32>>().0, random::<Closed01<f32>>().0, 1.0);
        let bounds = LayoutRect::new(
            LayoutPoint::new(layout.left, layout.top),
            LayoutSize::new(layout.width, layout.height)
        );
        debug!("layout: {:?}", layout);
        debug!("bounds: {:?}", bounds);

        builder.push_rect(bounds, None, color);

        for child in node.children() {
            self.build_display_list_recursive(builder, child);
        }
    }

    fn get_layout_node(&self, node: &Component) -> Ref<layout::Node> {
        self.layout_nodes.get(node.id()).unwrap().borrow()
    }

    fn get_layout_node_mut(&self, node: &Component) -> RefMut<layout::Node> {
        self.layout_nodes.get(node.id()).unwrap().borrow_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use euclid::size2;
    use component::panel;
    use layout::*;

    #[test]
    fn can_build_list() {
        let tree = panel(vec![
            FlexStyle::Width(1000.point()),
            FlexStyle::Height(1000.point()),
            FlexStyle::FlexDirection(FlexDirection::Row)
        ], vec![
            panel(vec![
                FlexStyle::Width(30.percent()),
                FlexStyle::Height(100.percent())
            ], vec![]),
            panel(vec![
                FlexStyle::Width(70.percent()),
                FlexStyle::Height(100.percent())
            ], vec![]),
        ]);

        let size = size2(1000.0, 1000.0);
        let mut theme = Theme::new();
        theme.update_layout(&tree, &size);

        let sizes = vec![
            theme.get_layout(&tree),
            theme.get_layout(&tree.children()[0]),
            theme.get_layout(&tree.children()[1]),
        ];

        assert_eq!(sizes, vec![
            Layout { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0, width: 1000.0, height: 1000.0 },
            Layout { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0, width: 300.0, height: 1000.0 },
            Layout { left: 300.0, right: 0.0, top: 0.0, bottom: 0.0, width: 700.0, height: 1000.0 },
        ]);
    }
}