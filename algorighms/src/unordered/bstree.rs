use std::{cmp::Ordering, collections::VecDeque, fmt::Debug};

/// An implementation of Binary Search Tree
#[derive(Debug, Default)]
pub struct BSTree<T> {
    root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Debug> BSTree<T> {
    /// Makes a new, empty `BSTree`
    pub fn new() -> Self {
        BSTree { root: None }
    }

    /// Makes a `BSTree` from a vector
    pub fn from_vec(arr: Vec<T>) -> Self {
        let mut tree = BSTree::new();
        for i in arr {
            tree.insert(i);
        }
        tree
    }

    /// Inserts an element to the tree
    pub fn insert(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None
        });

        match self.root {
            Some(ref mut node) => Self::insert_recursive(node, new_node),
            None => self.root = Some(new_node),
        }
    }

    /// Checks, if the element exists in the tree
    pub fn search(&self, value: T) -> bool {
        Self::search_recursive(&self.root, value)
    }

    pub fn print_tree(&self) {
        if let Some(ref root) = self.root {
            let tree_str = Self::print_recursive(root, 0);
            println!("{}", tree_str);
        }
    }

    pub fn dfs(&self) -> Option<Vec<&T>> {
        if let Some(ref node) = self.root {
            let mut res = Vec::new();
            Self::dfs_recursive(node, &mut res);
            Some(res)
        } else {
            None
        }
    }

    pub fn bsf(&self) -> Option<Vec<&T>> {
        let mut res = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(ref root) = self.root {
            queue.push_back(root);

            while let Some(ref node) = queue.pop_front() {
                res.push(&node.value);

                if let Some(ref left) = node.left {
                    queue.push_back(left);
                }

                if let Some(ref right) = node.right {
                    queue.push_back(right);
                }
            }
            Some(res)
        } else {
            None
        }
    }

    fn insert_recursive(current: &mut Box<Node<T>>, new_node: Box<Node<T>>) {
        if new_node.value < current.value {
            if let Some(ref mut left) = current.left {
                Self::insert_recursive(left, new_node);
            } else {
                current.left = Some(new_node)
            }
        } else if let Some(ref mut right) = current.right {
            Self::insert_recursive(right, new_node);
        } else {
            current.right = Some(new_node)
        }
    }

    fn search_recursive(node: &Option<Box<Node<T>>>, value: T) -> bool {
        match node {
            Some(n) => {
                match value.cmp(&n.value) {
                    Ordering::Equal => true,
                    Ordering::Less => Self::search_recursive(&n.left, value),
                    Ordering::Greater => Self::search_recursive(&n.right, value)
                }
            }
            None => false
        }
    }

    fn print_recursive(node: &Box<Node<T>>, depth: usize) -> String {
        let mut result = String::new();

        if let Some(ref left) = node.left {
            result.push_str(&Self::print_recursive(left, depth + 1));
        }

        result.push_str(&"\t".repeat(depth));
        result.push_str(&format!("{:?}\n", node.value));

        if let Some(ref right) = node.right {
            result.push_str(&Self::print_recursive(right, depth + 1));
        }

        result
    }

    fn dfs_recursive<'a, 'b: 'a>(node: &'b Box<Node<T>>, v: &'a mut Vec<&'b T>) {
        v.push(&node.value);

        if let Some(ref left) = node.left {
            Self::dfs_recursive(left, v);
        }

        if let Some(ref right) = node.right {
            Self::dfs_recursive(right, v);
        }
    }
}
