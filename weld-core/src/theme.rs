use webrender_api::*;
use component_tree::ComponentTree;
use layout::*;
use std::collections::HashMap;

pub struct Theme {
    layout: Box<Layout>
}

impl Theme {
    pub fn new() -> Theme {
        Theme {
            layout: CassowaryLayout::new()
        }
    }

    pub fn build_display_list(&self, builder: &mut DisplayListBuilder, tree: &ComponentTree, size: &LayoutSize) {
        /*let bounds = LayoutRect::new(LayoutPoint::new(0.0, 0.0), LayoutSize::new(100.0, 100.0));
        builder.push_rect(bounds, bounds, ColorF::new(1.0, 0.5, 1.0, 1.0));
        let clip = bounds.clone().inflate(-25.0, -25.0);
        let complex = ComplexClipRegion::new(clip, BorderRadius::uniform(0.0));
        builder.push_clip_node(None, clip, clip, vec![complex], None);
        builder.push_rect(bounds, bounds, ColorF::new(1.0, 1.0, 1.0, 1.0));
        builder.pop_clip_node();*/
        let bounds = LayoutRect::new(LayoutPoint::new(0.0, 0.0), size.clone());

        let mut sizes = HashMap::new();
        self.layout.determine_sizes(tree, bounds, &mut sizes);
    }
}