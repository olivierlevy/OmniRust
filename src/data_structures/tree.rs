// src/data_structures/tree.rs

//! # Generic Tree Data Structure
//!
//! This module provides a generic tree data structure (`Tree` and `TreeNode`).
//! Nodes store values of a generic type `T` and can have multiple children.
//! The tree uses `Rc<RefCell<TreeNode<T>>>` to allow for shared ownership and
//! interior mutability, enabling a node to be owned by its parent and potentially
//! referenced elsewhere, while still allowing modifications to its children list.
//!
//! ## Features
//! - Generic over the type of value stored in nodes (`T`).
//! - Nodes can have multiple children.
//! - Shared ownership of nodes via `Rc`.
//! - Interior mutability for children via `RefCell`.
//! - Basic operations: creating nodes, adding children, creating a tree with a root.
//! - Implements `Debug` and `Display` (for a simple textual representation).
//!
//! ## Usage
//!
//! ```
//! use omnirust::data_structures::tree::{Tree, TreeNode}; // Adjust path as per your project
//! use std::rc::Rc;
//! use std::cell::RefCell;
//!
//! // Create a new tree with a root value
//! let mut tree = Tree::with_root("root_value".to_string());
//!
//! if let Some(root_node_rc) = &tree.root {
//!     // Add children to the root node
//!     let mut root_node_mut = root_node_rc.borrow_mut();
//!     let child1_rc = root_node_mut.add_child("child1_value".to_string());
//!     let child2_rc = root_node_mut.add_child("child2_value".to_string());
//!
//!     // Add a grandchild
//!     child1_rc.borrow_mut().add_child("grandchild_value".to_string());
//! }
//!
//! // Print the tree structure
//! println!("{}", tree);
//! ```
//!
//! ## Considerations
//! - This implementation does not include parent pointers to avoid `Rc` cycles.
//!   If parent pointers are needed, `Weak<RefCell<TreeNode<T>>>` should be used,
//!   and care must be taken to manage lifetimes and potential cycles.
//! - For very large trees or performance-critical scenarios, other representations
//!   or memory management strategies might be more suitable.

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::rc::Rc;
use std::cell::RefCell;

/// Represents a node in the `Tree`.
///
/// Each `TreeNode` holds a `value` of generic type `T` and a vector of
/// `children`. Children are also `TreeNode`s, wrapped in `Rc<RefCell<...>>`
/// to allow shared ownership and interior mutability.
#[derive(Debug)]
pub struct TreeNode<T: Display + Debug> {
    /// The value stored in this node.
    pub value: T,
    /// A vector of child nodes. Each child is an `Rc<RefCell<TreeNode<T>>>`.
    pub children: Vec<Rc<RefCell<TreeNode<T>>>>,
    // pub parent: Option<Weak<RefCell<TreeNode<T>>>>, // For parent pointer if needed
}

impl<T: Display + Debug> TreeNode<T> {
    /// Creates a new tree node with the given value and no children.
    ///
    /// # Arguments
    /// * `value` - The value to store in the new node.
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
            // parent: None,
        }
    }

    /// Adds a new child to this node with the given value.
    ///
    /// A new `TreeNode` is created with `child_value`, wrapped in `Rc<RefCell<...>>`,
    /// and added to this node's children.
    ///
    /// # Arguments
    /// * `child_value` - The value for the new child node.
    ///
    /// # Returns
    /// An `Rc<RefCell<TreeNode<T>>>` pointing to the newly created child node,
    /// allowing further modifications or additions of grandchildren.
    pub fn add_child(&mut self, child_value: T) -> Rc<RefCell<TreeNode<T>>> {
        let child_node = Rc::new(RefCell::new(TreeNode::new(child_value)));
        self.children.push(Rc::clone(&child_node));
        // If parent pointer is needed:
        // child_node.borrow_mut().parent = Some(Rc::downgrade(&Rc::new(RefCell::new(self)))); // This is tricky with lifetimes
        child_node
    }

    /// Adds an existing `TreeNode` (already wrapped in `Rc<RefCell<...>>`) as a child.
    ///
    /// # Arguments
    /// * `child_node` - An `Rc<RefCell<TreeNode<T>>>` pointing to the node to be added as a child.
    pub fn add_child_node(&mut self, child_node: Rc<RefCell<TreeNode<T>>>) {
        self.children.push(child_node);
    }
}

/// Represents a generic tree structure.
///
/// The `Tree` itself is a simple wrapper around an optional root node.
/// If `root` is `None`, the tree is empty. Otherwise, it points to the
/// root `TreeNode` of the tree, wrapped in `Rc<RefCell<...>>`.
#[derive(Debug)]
pub struct Tree<T: Display + Debug> {
    /// The root node of the tree. `None` if the tree is empty.
    pub root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Display + Debug> Tree<T> {
    /// Creates a new, empty `Tree`.
    /// The `root` will be `None`.
    pub fn new() -> Self {
        Tree { root: None }
    }

    /// Creates a new `Tree` with a root node initialized with the given value.
    ///
    /// # Arguments
    /// * `value` - The value for the root node.
    pub fn with_root(value: T) -> Self {
        Tree {
            root: Some(Rc::new(RefCell::new(TreeNode::new(value)))),
        }
    }

    /// Sets or replaces the root node of the tree with a new node containing the given value.
    ///
    /// # Arguments
    /// * `value` - The value for the new root node.
    ///
    /// # Returns
    /// An `Rc<RefCell<TreeNode<T>>>` pointing to the new root node.
    pub fn set_root(&mut self, value: T) -> Rc<RefCell<TreeNode<T>>> {
        let new_root = Rc::new(RefCell::new(TreeNode::new(value)));
        self.root = Some(Rc::clone(&new_root));
        new_root
    }

    // Helper function for recursive display.
    // This function is private and used by the `Display` implementation.
    fn display_recursive(node: &Rc<RefCell<TreeNode<T>>>, f: &mut Formatter<'_>, depth: usize) -> FmtResult {
        writeln!(f, "{}{}", "  ".repeat(depth), node.borrow().value)?;
        for child in &node.borrow().children {
            Self::display_recursive(child, f, depth + 1)?;
        }
        Ok(())
    }
}

impl<T: Display + Debug> Default for Tree<T> {
    /// Creates a new, empty `Tree`. Equivalent to `Tree::new()`.
    fn default() -> Self {
        Self::new()
    }
}

/// Implements the `Display` trait for `Tree`.
/// This allows the tree to be printed in a human-readable format,
/// showing its hierarchical structure.
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
        let tree = Tree::with_root("FileSystem".to_string());
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
