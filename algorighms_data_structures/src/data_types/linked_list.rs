use std::ops::Deref;

#[derive(Debug)]
pub struct LinkedList<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}

#[allow(dead_code)]
impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, val: T) {
        let mut new_node = Box::new(Node::new(val));
        let new_node_ptr = &mut *new_node as *mut Node<T>;
        
        if self.head.is_none() {
            self.head = Some(new_node);
            self.tail = Some(new_node_ptr);
        } else {
            unsafe {
                if let Some(tail_ptr) = self.tail {
                    (*tail_ptr).next = Some(new_node);
                    self.tail = Some(new_node_ptr);
                }
            }
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

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LLIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        LLIntoIter {
            current: self.head
        }
    }
}

pub struct LLIntoIter<T> {
    current: Option<Box<Node<T>>>
}

impl<T> Iterator for LLIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.current.take() {
            let next_node = n.next;
            self.current = next_node;
            return Some(n.value);
        }
        None
    }
}
