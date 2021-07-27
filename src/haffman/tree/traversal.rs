use std::collections::HashMap;

use super::structs::*;

#[derive(Clone, Copy, Debug)]
pub enum ChildPosition {
    Left,
    Right,
}

impl<T: Clone + Eq + std::hash::Hash + Copy> Tree<T> {
    pub fn deep_first_traversal(&self) -> HashMap<Box<TreeLeaf<T>>, Vec<ChildPosition>> {
        let path = Vec::<ChildPosition>::new();
        let mut paths_map = HashMap::<Box<TreeLeaf<T>>, Vec<ChildPosition>>::new();
        Tree::deep_first_recursion(self, path, &mut paths_map);

        paths_map
    }

    fn deep_first_recursion(
        root: &Tree<T>,
        path: Vec<ChildPosition>,
        paths_map: &mut HashMap<Box<TreeLeaf<T>>, Vec<ChildPosition>>,
    ) {
        match root {
            Tree::Node(node) => {
                let mut left_path = path.clone();
                left_path.push(ChildPosition::Left);
                let mut right_path = path.clone();
                right_path.push(ChildPosition::Right);
                Tree::deep_first_recursion(&node.left, left_path, paths_map);
                Tree::deep_first_recursion(&node.right, right_path, paths_map);
            }
            Tree::Leaf(leaf) => {
                paths_map.insert(leaf.clone(), path);
            }
            Tree::None => {
                //Proceed
            }
        }
    }

    pub fn get_and_create(&mut self, position: ChildPosition) -> Tree<T> {
        match self {
            Tree::Node(self_node) => match position {
                ChildPosition::Left => {
                    self_node.left = Tree::match_subtree(&self_node.left, true);
                    self_node.left.clone()
                }
                ChildPosition::Right => {
                    self_node.right = Tree::match_subtree(&self_node.right, true);
                    self_node.right.clone()
                }
            },
            _ => {
                panic!("Paths are applicable only for nodes")
            }
        }
    }

    fn match_subtree(node: &Tree<T>, create_new_on_none: bool) -> Tree<T> {
        match node {
            Tree::None => {
                if create_new_on_none {
                    Tree::new_node(Tree::None, Tree::None)
                } else {
                    Tree::None
                }
            }
            Tree::Node(_) => node.clone(),
            Tree::Leaf(_) => panic!("Bad path while matching subtree"),
        }
    }

    // pub fn follow_path(&self, path: Vec<ChildPosition>) -> Tree<T> {
    //     let mut curr = self.clone();
    //     for position in path {
    //         match curr {
    //             Tree::Node(node) => curr = Tree::get_child(&node, position),
    //             _ => {
    //                 panic!("Bad path")
    //             }
    //         }
    //     }

    //     curr.clone()
    // }

    // fn get_child(node: &TreeNode<T>, position: ChildPosition) -> Tree<T> {
    //     match position {
    //         ChildPosition::Left => Tree::match_subtree(&node.left, false),
    //         ChildPosition::Right => Tree::match_subtree(&node.right, false),
    //     }
    // }

    pub fn get_child(&self, position: ChildPosition) -> Tree<T> {
        match self {
            Tree::Node(node) => match position {
                ChildPosition::Left => Tree::match_subtree(&node.left, false),
                ChildPosition::Right => Tree::match_subtree(&node.right, false),
            },
            _ => {
                panic!("Bad path, unable to get child at position")
            }
        }
    }
}
