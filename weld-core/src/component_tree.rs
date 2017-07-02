use id_tree;
use id_tree::{NodeId, Tree, TreeBuilder};
use components::component::*;

pub struct ComponentTree {
    tree: Tree<Box<Component>>,
}

impl ComponentTree {
    pub fn new() -> ComponentTree {
        ComponentTree { tree: TreeBuilder::new().build() }
    }

    pub fn add_node<T: Into<Component>>(&mut self,
                                        component: T,
                                        parent: Option<&NodeId>)
                                        -> NodeId {
        let boxed = Box::new(component.into());

        match parent {
            Some(parent_id) => {
                self.tree
                    .insert(id_tree::Node::new(boxed),
                            id_tree::InsertBehavior::UnderNode(parent_id))
                    .unwrap()
            }
            None => {
                self.tree
                    .insert(id_tree::Node::new(boxed), id_tree::InsertBehavior::AsRoot)
                    .unwrap()
            }
        }
    }

    pub fn get(&self, id: &NodeId) -> &Component {
        self.tree.get(id).unwrap().data().as_ref()
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use component::Component;

    #[test]
    fn test_tree() {
        let mut tree = ComponentTree::new();
        let root = Component::new();
        let root_id = tree.add_node(root, None);
    }
}*/