use webrender::api::*;
use model::{Component, ComponentId, InspectableComponent, RenderContext, RenderElement};
use std::collections::HashMap;
use std::cell::{Ref, RefMut, RefCell};
use layout;

pub struct LayoutContext {
    layout_nodes: HashMap<ComponentId, RefCell<layout::Node>>
}

impl LayoutContext {
    pub fn new() -> LayoutContext {
        LayoutContext {
            layout_nodes: HashMap::new()
        }
    }

    pub fn get_layout(&self, node: &InspectableComponent) -> layout::Layout {
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
            let layout_node = self.layout_nodes.entry(*node.inspect().id()).or_insert_with(|| RefCell::new(layout::Node::new()));
            layout_node.borrow_mut().apply_styles(node.inspect().styles());
        }

        for child in node.inspect().children() {
            self.update_layout_recursive(child);

            let mut layout_node = self.get_layout_node_mut(node);
            let mut layout_child = self.get_layout_node_mut(child);
            let child_count = layout_node.child_count();
            layout_node.insert_child(&mut layout_child, child_count);
        }
    }

    pub fn build_display_list(&self, builder: &mut DisplayListBuilder, root: &Component) {
        let mut elements = Vec::new();
        {
            let mut ctx = WebrenderRenderContext::new(&self, root, &mut elements);
            root.inspect().renderer().render(&mut ctx);
        }

        for element in elements {
            match element {
                RenderElement::Rect(rect, color) => {
                    builder.push_rect(rect, None, color);
                }
            }
        }
    }

    fn get_layout_node(&self, node: &InspectableComponent) -> Ref<layout::Node> {
        self.layout_nodes.get(node.id()).unwrap().borrow()
    }

    fn get_layout_node_mut(&self, node: &InspectableComponent) -> RefMut<layout::Node> {
        self.layout_nodes.get(node.id()).unwrap().borrow_mut()
    }

    pub fn find_node_at<'a>(&self, point: WorldPoint, root: &'a Component) -> Option<&'a Component> {
        self.find_node_at_recursive(point, root)
    }

    fn find_node_at_recursive<'a>(&self, point: WorldPoint, node: &'a Component) -> Option<&'a Component> {
        let layout = self.get_layout(node);
        let rect = WorldRect::new(WorldPoint::new(layout.left, layout.top), WorldSize::new(layout.width, layout.height));
        if !rect.contains(&point) {
            None
        } else {
            for child_id in node.inspect().children() {
                if let Some(found_in_child) = self.find_node_at_recursive(point, child_id) {
                    return Some(found_in_child);
                }
            }

            Some(node)
        }
    }
}

struct WebrenderRenderContext<'a> {
    layout_context: &'a LayoutContext,
    component: &'a InspectableComponent,
    elements: &'a mut Vec<RenderElement>,
}

impl<'a> WebrenderRenderContext<'a> {
    pub fn new(layout_context: &'a LayoutContext, component: &'a InspectableComponent, elements: &'a mut Vec<RenderElement>) -> WebrenderRenderContext<'a> {
        WebrenderRenderContext {
            layout_context,
            component,
            elements,
        }
    }
}

impl<'a> RenderContext for WebrenderRenderContext<'a> {
    fn render(&mut self) {
        self.component.renderer().render(self);
    }

    fn push(&mut self, e: RenderElement) {
        self.elements.push(e);
    }

    fn next(&mut self) {
        for child in self.component.children().iter() {
            let mut child_context = WebrenderRenderContext::new(self.layout_context, child.inspect(), self.elements);
            child_context.render();
        }
    }

    fn bounds(&self) -> LayoutRect {
        let layout = &self.layout_context.get_layout(self.component);

        LayoutRect::new(
            LayoutPoint::new(layout.left, layout.top),
            LayoutSize::new(layout.width, layout.height)
        )
    }
}
