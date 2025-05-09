// src/data_structures/tree.rs

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::rc::Rc;
use std::cell::RefCell;

// A node in the tree.
#[derive(Debug)]
pub struct TreeNode<T: Display + Debug> {
    pub value: T,
    pub children: Vec<Rc<RefCell<TreeNode<T>>>>,
    // pub parent: Option<Weak<RefCell<TreeNode<T>>>>, // For parent pointer if needed
}

impl<T: Display + Debug> TreeNode<T> {
    /// Creates a new tree node with the given value.
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
            // parent: None,
        }
    }

    /// Adds a child node to this node.
    pub fn add_child(&mut self, child_value: T) -> Rc<RefCell<TreeNode<T>>> {
        let child_node = Rc::new(RefCell::new(TreeNode::new(child_value)));
        self.children.push(Rc::clone(&child_node));
        // If parent pointer is needed:
        // child_node.borrow_mut().parent = Some(Rc::downgrade(&Rc::new(RefCell::new(self)))); // This is tricky with lifetimes
        child_node
    }

    /// Adds an existing TreeNode (wrapped in Rc<RefCell<>>) as a child.
    pub fn add_child_node(&mut self, child_node: Rc<RefCell<TreeNode<T>>>) {
        self.children.push(child_node);
    }
}

// A simple Tree structure, essentially a wrapper around the root node.
#[derive(Debug)]
pub struct Tree<T: Display + Debug> {
    pub root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Display + Debug> Tree<T> {
    /// Creates a new empty tree.
    pub fn new() -> Self {
        Tree { root: None }
    }

    /// Creates a new tree with a root node having the given value.
    pub fn with_root(value: T) -> Self {
        Tree {
            root: Some(Rc::new(RefCell::new(TreeNode::new(value)))),
        }
    }

    /// Sets the root of the tree.
    pub fn set_root(&mut self, value: T) -> Rc<RefCell<TreeNode<T>>> {
        let new_root = Rc::new(RefCell::new(TreeNode::new(value)));
        self.root = Some(Rc::clone(&new_root));
        new_root
    }

    // Helper function for recursive display (optional)
    fn display_recursive(node: &Rc<RefCell<TreeNode<T>>>, f: &mut Formatter<'_>, depth: usize) -> FmtResult {
        writeln!(f, "{}{}", "  ".repeat(depth), node.borrow().value)?;
        for child in &node.borrow().children {
            Self::display_recursive(child, f, depth + 1)?;
        }
        Ok(())
    }
}

impl<T: Display + Debug> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Display for a nice printout of the tree (optional)
impl<T: Display + Debug> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.root {
            Some(root_node) => Self::display_recursive(root_node, f, 0),
            None => write!(f, "Empty Tree"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_new() {
        let node = TreeNode::new(10);
        assert_eq!(node.value, 10);
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_add_child() {
        let mut root_node_val = TreeNode::new("root".to_string());
        let child1_rc = root_node_val.add_child("child1".to_string());
        root_node_val.add_child("child2".to_string());

        assert_eq!(root_node_val.children.len(), 2);
        assert_eq!(child1_rc.borrow().value, "child1");
        assert_eq!(root_node_val.children[0].borrow().value, "child1");
        assert_eq!(root_node_val.children[1].borrow().value, "child2");
    }

    #[test]
    fn test_tree_with_root() {
        let tree = Tree::with_root("Root Node");
        assert!(tree.root.is_some());
        assert_eq!(tree.root.as_ref().unwrap().borrow().value, "Root Node");
    }

    #[test]
    fn test_tree_structure_and_display() {
        let mut tree = Tree::with_root("FileSystem".to_string());
        if let Some(root) = &tree.root {
            let mut root_mut = root.borrow_mut();
            let usr_rc = root_mut.add_child("usr".to_string());
            let home_rc = root_mut.add_child("home".to_string());

            {
                let mut usr_node_mut = usr_rc.borrow_mut();
                usr_node_mut.add_child("bin".to_string());
                usr_node_mut.add_child("lib".to_string());
            }

            {
                let mut home_node_mut = home_rc.borrow_mut();
                let user1_rc = home_node_mut.add_child("user1".to_string());
                {
                    user1_rc.borrow_mut().add_child("docs".to_string());
                }
            }
        }

        let display_str = format!("{}", tree);
        println!("Tree Display:\n{}", display_str); // For manual inspection during test run

        assert!(display_str.contains("FileSystem"));
        assert!(display_str.contains("  usr"));
        assert!(display_str.contains("    bin"));
        assert!(display_str.contains("  home"));
        assert!(display_str.contains("    user1"));
        assert!(display_str.contains("      docs"));
    }

    #[test]
    fn test_empty_tree() {
        let tree: Tree<i32> = Tree::new();
        assert!(tree.root.is_none());
        assert_eq!(format!("{}", tree), "Empty Tree");
    }

    #[test]
    fn test_set_root() {
        let mut tree: Tree<String> = Tree::new();
        assert!(tree.root.is_none());
        tree.set_root("New Root".to_string());
        assert!(tree.root.is_some());
        assert_eq!(tree.root.as_ref().unwrap().borrow().value, "New Root");
    }
}
