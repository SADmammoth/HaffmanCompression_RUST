use super::structs::*;

impl<T: Copy> Tree<T> {
    pub fn new_node(left: Tree<T>, right: Tree<T>) -> Tree<T> {
        let priority = left.get_priority() + right.get_priority();
        Tree::Node(Box::new(TreeNode {
            left,
            right,
            priority,
        }))
    }

    pub fn new_leaf(content: T, priority: u128) -> Tree<T> {
        Tree::Leaf(Box::new(TreeLeaf { content, priority }))
    }

    pub fn get_priority(&self) -> u128 {
        match self {
            Tree::Node(node) => node.priority,
            Tree::Leaf(leaf) => leaf.priority,
            Tree::None => 0,
        }
    }

    pub fn get_content(&self) -> Option<T> {
        match self {
            Tree::Node(_) => None,
            Tree::Leaf(leaf) => Some(leaf.content),
            Tree::None => None,
        }
    }

    pub fn set_content(&mut self, content: T) {
        match self {
            Tree::Leaf(leaf) => leaf.content = content,
            _ => {}
        }
    }

    pub fn is_leaf(&self) -> bool {
        if let Tree::Leaf(_) = self {
            true
        } else {
            false
        }
    }

    pub fn set_left(&mut self, left: Tree<T>) {
        if let Tree::Node(node) = self {
            node.left = left;
        } else {
            panic!("Unable to set left")
        }
    }

    pub fn set_right(&mut self, right: Tree<T>) {
        if let Tree::Node(node) = self {
            node.right = right;
        } else {
            panic!("Unable to set left")
        }
    }

    pub fn get_or_create_left(&mut self) -> &mut Tree<T> {
        match self {
            Tree::Node(node) => match &node.left {
                Tree::Node(_) => &mut node.left,
                Tree::None => {
                    node.left = Tree::new_node(Tree::None, Tree::None);
                    &mut node.left
                }
                Tree::Leaf(_) => {
                    panic!("Error")
                }
            },
            _ => {
                panic!("Error")
            }
        }
    }

    pub fn get_or_create_right(&mut self) -> &mut Tree<T> {
        match self {
            Tree::Node(node) => match &node.right {
                Tree::Node(_) => &mut node.right,
                Tree::None => {
                    node.right = Tree::new_node(Tree::None, Tree::None);
                    &mut node.right
                }
                Tree::Leaf(_) => {
                    panic!("Error")
                }
            },
            _ => {
                panic!("Error")
            }
        }
    }

    pub fn create_left_leaf(&mut self, content: T, priority: u128) {
        match self {
            Tree::Node(node) => node.left = Tree::new_leaf(content, priority),
            _ => {
                panic!("Error")
            }
        }
    }

    pub fn create_right_leaf(&mut self, content: T, priority: u128) {
        match self {
            Tree::Node(node) => node.right = Tree::new_leaf(content, priority),
            _ => {
                panic!("Error")
            }
        }
    }
}
