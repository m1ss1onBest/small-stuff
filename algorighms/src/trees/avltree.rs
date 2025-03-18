use core::panic;
use std::cmp::{max, Ordering};


type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct AvlTree<T> {
    root: Link<T>,
}

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    height: i32,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord + Clone> AvlTree<T> {
    pub fn new() -> Self {
        AvlTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_recursive(self.root.take(), value);
    }

    pub fn find(&self, value: &T) -> bool {
        self.search(value)
    }

    pub fn remove(&mut self, value: T) {
        self.root = Self::delete(self.root.take(), value);
    }

    fn height(node: &Link<T>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn balance_factor(node: &Link<T>) -> i32 {
        match node {
            None => 0,
            Some(n) => Self::height(&n.left) - Self::height(&n.right)
        }
    }

    fn update_height(node: &mut Box<Node<T>>) {
        node.height = max(Self::height(&node.left), Self::height(&node.right)) + 1;
    }

    fn rotate_right(mut y: Box<Node<T>>) -> Box<Node<T>> {
        let mut x = y.left.take().unwrap();
        y.left = x.right.take();
        Self::update_height(&mut y);
        x.right = Some(y);
        Self::update_height(&mut x);
        x
    }

    fn rotate_left(mut x: Box<Node<T>>) -> Box<Node<T>> {
        let mut y = x.right.take().unwrap();
        x.right = y.left.take();
        Self::update_height(&mut x);
        y.left = Some(x);
        y
    }

    fn balance(mut node: Box<Node<T>>) -> Box<Node<T>> {
        Self::update_height(&mut node);
        let balance = Self::balance_factor(&Some(node.clone()));

        // Left Heavy
        if balance > 1 {
            if Self::balance_factor(&node.left) < 0 {
                node.left = Some(Self::rotate_left(node.left.take().unwrap()));
            }
            return Self::rotate_right(node);
        }
        if balance < -1 {
            if Self::balance_factor(&node.right) > 0 {
                node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            }
            return Self::rotate_left(node);
        }
        node
    }

    fn insert_recursive(node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {
        let old_value = value.clone();
        let mut node = match node {
            None => Box::new(Node {
                value,
                height: 1,
                left: None,
                right: None,
            }),
            Some(n) => n,
        };

        match old_value.cmp(&node.value) {
            Ordering::Less => {
                node.left = Self::insert_recursive(node.left.take(), old_value);
            }
            Ordering::Greater => {
                node.right = Self::insert_recursive(node.right.take(), old_value);
            }
            Ordering::Equal => return Some(node),
        }

        Some(Self::balance(node))
    }

    fn search(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return true,
            }
        }
        false
    }

    fn min_value_node(node: &Link<T>) -> &T {
        let mut current = node;
        while let Some(n) = current {
            if n.left.is_none() {
                return &n.value;
            }
            current = &n.left;
        }
        panic!("unreachable code");
    }

    fn delete(node: Link<T>, value: T) -> Link<T> {
        let mut node = match node {
            None => return None,
            Some(n) => n,
        };
    
        match value.cmp(&node.value) {
            Ordering::Less => {
                node.left = Self::delete(node.left.take(), value);
            }
            Ordering::Greater => {
                node.right = Self::delete(node.right.take(), value);
            }
            Ordering::Equal => {
                if node.left.is_none() {
                    return node.right;
                } else if node.right.is_none() {
                    return node.left;
                } else {
                    let mut right_subtree = node.right.take();
                    if let Some(ref mut right_node) = right_subtree {

                        let temp_val = Some(right_node.clone());
                        let min_value = Self::min_value_node(&temp_val);
                        node.value = min_value.clone();
                        right_subtree = Self::delete(right_subtree, min_value.clone());
                    }
                    node.right = right_subtree;
                }
            }
        }
    
        Some(Self::balance(node))
    }
}