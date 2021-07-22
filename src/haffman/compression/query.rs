use super::super::tree::Tree;
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter, Result};

pub struct Query<T>(pub BinaryHeap<Tree<T>>);

impl<T: Display + Ord + Clone + Copy> Display for Query<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            self.0
                .clone()
                .into_sorted_vec()
                .iter()
                .map(|x| { x.stringify() })
                .collect::<String>()
        )
    }
}
