use std::fmt::Display;

pub struct LList<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    size: usize,
}

impl<T> LList<T> {
    pub fn new() -> LList<T> {
        LList {
            head: None,
            tail: std::ptr::null_mut(),
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_node = Node::boxed(value);

        if self.head.is_none() {
            let raw_node = &mut *new_node as *mut Node<T>;
            self.tail = raw_node;
        }

        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let mut new_node = Node::boxed(value);
        let raw_node = &mut *new_node as *mut Node<T>;

        if self.head.is_none() {
            self.head = Some(new_node);
            self.tail = raw_node;
        } else {
            unsafe { (*self.tail).next = Some(new_node) };
            self.tail = raw_node;
        }
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            if self.head.is_none() {
                self.tail = std::ptr::null_mut()
            }
            self.size -= 1;
            node.value
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if self.head.as_ref()?.next.is_none() {
            return self.pop_front();
        }

        let mut current = self.head.as_mut()?;
        while current.next.as_ref()?.next.is_some() {
            current = current.next.as_mut()?;
        }
        let last = current.next.take()?;
        self.tail = &mut **current as *mut _;

        self.size -= 1;
        Some(last.value)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn print(&self)
    where 
        T: Display {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn boxed(value: T) -> Box<Node<T>> {
        Box::new(Node { next: None, value })
    }
}
