use super::Vector;

pub struct LQueue<T> {
    inner: Vector<T>,
}

impl<T> LQueue<T> {
    pub fn new() -> LQueue<T> {
        LQueue {
            inner: Vector::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }

    pub fn push(&mut self, val: T) {
        self.inner.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    pub fn pop_last(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.inner.len() == usize::MAX
    }
}