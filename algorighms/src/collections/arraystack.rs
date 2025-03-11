use super::Vector;

pub struct ArrayStack<T> {
    inner: Vector<T>,
}

impl<T> ArrayStack<T> {
    pub fn new() -> ArrayStack<T> {
        ArrayStack {
            inner: Vector::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.inner.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }
   
    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.inner.len() == usize::MAX
    }

    pub fn drain(&mut self) {
        self.inner = Vector::new()
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }
}