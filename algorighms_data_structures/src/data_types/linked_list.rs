use std::{ops::Deref, ptr::NonNull, rc::Rc};

#[derive(Debug)]
pub struct LinkedList<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            length: 0,
            head: None,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, val: T) {
        let new_val = Box::new(Node::new(val));

        if let Some(mut node) = self.head.take() {
            let mut next_node = &mut node.next;
            while let Some(new_node) = next_node {
                next_node = &mut new_node.next;
            }
            *next_node = Some(new_val);
            self.head = Some(node);
        }
        //list is empty
        else {
            self.head = Some(new_val);
        }
        self.length += 1;
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList::new()
    }
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            next: None,
        }
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
