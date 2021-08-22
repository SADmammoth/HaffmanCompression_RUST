use std::collections::HashMap;

use super::structs::*;

#[derive(Clone, Copy, Debug)]
pub enum ChildPosition {
    Left,
    Right,
}

impl<T: Clone + Eq + std::hash::Hash + Copy> Tree<T> {
    pub fn deep_first_traversal(&self) -> HashMap<TreeLeaf<T>, Vec<ChildPosition>> {
        let path = Vec::<ChildPosition>::new();
        let mut paths_map = HashMap::<TreeLeaf<T>, Vec<ChildPosition>>::new();
        Tree::deep_first_recursion(self, path, &mut paths_map);

        paths_map
    }

    fn deep_first_recursion(
        root: &Tree<T>,
        path: Vec<ChildPosition>,
        paths_map: &mut HashMap<TreeLeaf<T>, Vec<ChildPosition>>,
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
                paths_map.insert(*leaf, path);
            }
            Tree::None => {
                //Proceed
            }
        }
    }
}
