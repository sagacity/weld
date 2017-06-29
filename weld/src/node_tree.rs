use id_tree;
use id_tree::{NodeId, Tree, TreeBuilder};
use node::Node;

pub struct NodeTree {
    tree: Tree<Node>,
}

impl NodeTree {
    pub fn new() -> NodeTree {
        NodeTree { tree: TreeBuilder::new().build() }
    }

    pub fn add_node(&mut self, node: Node, parent: Option<NodeId>) -> NodeId {
        match parent {
            Some(parent_id) => {
                self.tree
                    .insert(id_tree::Node::new(node),
                            id_tree::InsertBehavior::UnderNode(&parent_id))
                    .unwrap()
            }
            None => {
                self.tree.insert(id_tree::Node::new(node), id_tree::InsertBehavior::AsRoot).unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodetree() {
        let mut tree = NodeTree::new();
        let root = Node::new();
        let root_id = tree.add_node(root, None);
    }
}