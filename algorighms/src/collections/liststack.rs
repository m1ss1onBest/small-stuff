use super::LList;

pub struct ListStack<T> {
    inner: LList<T>,
}

impl<T> ListStack<T> {
        pub fn new() -> ListStack<T> {
            ListStack {
                inner: LList::new(),
            }
        }

        pub fn push(&mut self, val: T) {
            self.inner.push_front(val);
        }
    
        pub fn pop(&mut self) -> Option<T> {
            self.inner.pop_front()
        }
       
        pub fn is_empty(&self) -> bool {
            self.inner.size() == 0
        }
    
        pub fn is_full(&self) -> bool {
            self.inner.size() == usize::MAX
        }
    
        pub fn drain(&mut self) {
            self.inner = LList::new()
        }
    
        pub fn size(&self) -> usize {
            self.inner.size()
        }
}