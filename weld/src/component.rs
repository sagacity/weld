#![allow(dead_code)]
#![allow(unused_variables)]

use data_bag::DataBag;

#[derive(Debug, PartialEq)]
pub struct Size {
    width: f64,
    height: f64,
}

#[derive(Debug, PartialEq)]
pub struct Caption {
    caption: &'static str,
}

pub struct Component {
    data_bag: DataBag,
}

impl Component {
    fn new() -> Component {
        Component { data_bag: DataBag::new() }
    }
}

pub trait Label {
    fn get_caption(&self) -> Option<&Caption>;
}

impl Label for Component {
    fn get_caption(&self) -> Option<&Caption> {
        self.data_bag.get::<Caption>()
    }
}

pub struct LabelBuilder {
    caption: &'static str,
}

impl LabelBuilder {
    pub fn new() -> Self {
        Self { caption: "Untitled" }
    }

    pub fn caption(mut self, caption: &'static str) -> Self {
        self.caption = caption;
        self
    }
}

impl Into<Component> for LabelBuilder {
    fn into(self) -> Component {
        let mut c = Component::new();
        c.data_bag.put(Caption { caption: self.caption });
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use component_tree::ComponentTree;

    #[test]
    fn test_tree() {
        let mut tree = ComponentTree::new();

        let root = tree.add_node(LabelBuilder::new().caption("Parent"), None);
        let child = tree.add_node(LabelBuilder::new().caption("Child"), Some(&root));
        let child2 = tree.add_node(LabelBuilder::new().caption("Child2"), Some(&root));

        assert_eq!((tree.get(&root) as &Label).get_caption().unwrap().caption,
                   "Parent");
    }
}
