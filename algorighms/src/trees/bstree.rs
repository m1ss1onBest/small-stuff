use std::{cmp::Ordering, fmt::Debug};

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
    pub fn new() -> Self {
        BSTree { root: None }
    }

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

    pub fn search(&self, value: T) -> bool {
        Self::search_recursive(&self.root, value)
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

    pub fn print_tree(&self) {
        if let Some(ref root) = self.root {
            Self::print_recursive(root, 0);
        }
    }

    fn print_recursive(node: &Box<Node<T>>, depth: usize) {
        if let Some(ref left) = node.left {
            Self::print_recursive(left, depth + 1);
        }

        for _ in 0..depth {
        print!("    ");
        }
        println!("{:?}", node.value);

        if let Some(ref right) = node.right {
            Self::print_recursive(right, depth + 1);
        }
    }
}

