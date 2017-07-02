use webrender_traits::*;
use weld_core::component_tree::ComponentTree;

pub struct Theme;

impl Theme {
    pub fn new() -> Theme {
        Theme
    }

    pub fn build_display_list(&self, builder: &mut DisplayListBuilder, tree: &ComponentTree) {
        // dummy rect
        let bounds = LayoutRect::new(LayoutPoint::new(50.0, 50.0), LayoutSize::new(100.0, 100.0));
        builder.push_rect(bounds, bounds, ColorF::new(1.0, 1.0, 1.0, 1.0));
    }
}