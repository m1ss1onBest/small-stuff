use std::{cmp::Ordering, mem};

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub struct Map<K, V> {
    root: Link<K, V>,
    num_nodes: usize,
}

#[derive(Debug)]
pub struct Node<K, V> {
    key: K,
    value: V,
    color: Color,
    left: Link<K, V>,
    right: Link<K, V>,
}

#[derive(Debug)]
enum Color {
    Red,
    Black
}

impl Color {
    pub fn is_red(&self) -> bool {
        matches!(*self, Color::Red)
    }

    pub fn is_black(&self) -> bool {
        !self.is_red()
    }

    pub fn switch(&mut self) {
        *self = match self {
            Self::Red => Self::Black,
            Self::Black => Self::Red,
        }
    }
}

impl<K: Ord, V> Map<K, V> {
    pub fn new() -> Self {
        Map {
            root: None,
            num_nodes: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.num_nodes
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.num_nodes += 1;

        if let Some(ref mut node) = self.root {
            node.insert(key, value)
        } else {
            self.root = Node::new(key, value);
            None
        }
    }
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Link<K, V> {
        Some(Box::new(Node {
            key,
            value,
            color: Color::Red,
            left: None,
            right: None,
        }))
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match key.cmp(&self.key) {
            Ordering::Less => self.put(key, value, |n| &mut n.left),
            Ordering::Greater => self.put(key, value, |n| &mut n.right),
            Ordering::Equal => Some(mem::replace(&mut self.value, value)),
        }
    }

    //helper function to avoid code duplication
    fn put<F>(&mut self, key: K, value: V, mut f: F) -> Option<V>
    where
        F: FnMut(&mut Node<K, V>) -> &mut Link<K, V>,
   {
        let target = f(self);

        if let Some(ref mut node) = target {
            node.insert(key, value)
        } else {
            *target = Node::new(key, value);
            None
        }
    }
}
