use core::fmt;
use std::fmt::Display;

use super::structs::*;

impl<T: Display> Tree<T> {
    pub fn stringify(&self) -> String {
        let mut string = String::new();

        Tree::stringify_recursion(&mut string, self, 0, "").clone()
    }

    fn stringify_recursion<'a>(
        string: &'a mut String,
        root: &Tree<T>,
        indent: usize,
        add: &str,
    ) -> &'a String {
        match root {
            Tree::Node(node) => {
                string.push_str(&format!(
                    "{item:>indent$}\n",
                    item = format!("{}{}", add, node.priority),
                    indent = indent
                ));
                Tree::stringify_recursion(string, &node.right, indent + 6, "r: ");
                Tree::stringify_recursion(string, &node.left, indent + 6, "l: ");
            }
            Tree::Leaf(leaf) => {
                string.push_str(&format!(
                    "{item:>indent$}",
                    item = format!("{}{}", add, leaf.priority),
                    indent = indent
                ));
                string.push_str(&format!("({})\n", leaf.content));
            }
            Tree::None => {}
        };

        string
    }
}

impl<T: Display> Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stringify())
    }
}
