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
    fn new_label(caption: &'static str) -> Component {
        let mut bag = DataBag::new();
        bag.put(Caption { caption: caption });
        Component { data_bag: bag }
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

#[cfg(test)]
mod tests {
    use super::*;
    use component_tree::ComponentTree;

    #[test]
    fn test_tree() {
        let mut tree = ComponentTree::new();
        let root = tree.add_node(Component::new_label("Parent"), None);
        let child = tree.add_node(Component::new_label("Child"), Some(&root));
        let child2 = tree.add_node(Component::new_label("Child2"), Some(&root));

        assert_eq!((tree.get(&root) as &Label).get_caption().unwrap().caption,
                   "Parent");
    }
}
