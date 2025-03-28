use std::{cmp::Ordering, mem, ops::Index};

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
    #[allow(dead_code)]
    color: Color,
    left: Link<K, V>,
    right: Link<K, V>,
}

#[derive(Debug)]
enum Color {
    Red,
    Black
}

#[allow(private_interfaces)]
pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

struct OccupiedEntry<'a, K, V> {
    node: &'a mut Node<K, V>,
}

struct VacantEntry<'a, K, V> {
    key: K,
    parent: &'a mut Link<K, V>,
}

#[allow(unused)]
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

    pub fn get(&self, key: K) -> Option<&V> {
        self.root.as_ref()?.get(key)
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.root.as_mut()?.get_mut(key)
    }

    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        let mut current = &mut self.root;

        while let Some(ref mut node) = current {
            match key.cmp(&node.key) {
                Ordering::Less => current = &mut node.left,
                Ordering::Greater => current = &mut node.right,
                Ordering::Equal => {
                    return Entry::Occupied(OccupiedEntry { node })
                }
            }
        }
        Entry::Vacant(VacantEntry { key, parent: current })
    }
}

impl<K: Ord, V> Index<K> for Map<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.get(index).expect("entry not found by key")
    }
}

impl<K: Ord, V> Node<K, V> {
    fn new(key: K, value: V) -> Link<K, V> {
        Some(Box::new(Node {
            key,
            value,
            color: Color::Red,
            left: None,
            right: None,
        }))
    }

    fn get(&self, key: K) -> Option<&V> {
        match key.cmp(&self.key) {
            Ordering::Less => self.left.as_ref()?.get(key),
            Ordering::Greater => self.right.as_ref()?.get(key),
            Ordering::Equal => Some(&self.value),
        }
    }

    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        match key.cmp(&self.key) {
            Ordering::Less => self.left.as_mut()?.get_mut(key),
            Ordering::Greater => self.right.as_mut()?.get_mut(key),
            Ordering::Equal => Some(&mut self.value),
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match key.cmp(&self.key) {
            Ordering::Less => self.put(key, value, |n| &mut n.left),
            Ordering::Greater => self.put(key, value, |n| &mut n.right),
            Ordering::Equal => Some(mem::replace(&mut self.value, value)),
        }
    }

    // helper function to avoid code duplication
    fn put<F>(&mut self, key: K, value: V, f: F) -> Option<V>
    where
        F: FnOnce(&mut Node<K, V>) -> &mut Link<K, V>,
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

impl <'a, K: Ord, V> Entry<'a, K, V> {
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }
}

#[allow(unused)]
impl<'a, K: Ord, V> OccupiedEntry<'a, K, V> {
    pub fn get(&self) -> &V {
        &self.node.value
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.node.value
    }

    pub fn into_mut(self) -> &'a mut V {
        &mut self.node.value
    }
}

impl<'a, K: Ord, V> VacantEntry<'a, K, V> {
    pub fn insert(self, value: V) -> &'a mut V {
        *self.parent = Node::new(self.key, value);
        &mut self.parent.as_mut().unwrap().value
    }
}