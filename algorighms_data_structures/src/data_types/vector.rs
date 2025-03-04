use core::fmt;
use std::{array::IntoIter, clone, fmt::Debug, ops::Deref};

pub struct Vector<T> {
    data: Box<[T]>,
    length: usize,
    capacity: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Vector<T>
    where
        T: Default,
    {
        Vector {
            data: Box::new([T::default()]),
            length: 0,
            capacity: 1,
        }
    }

    pub fn with_capacity(capacity: usize) -> Vector<T>
    where 
        T: Default + Clone,
    {
        Vector {
            data: vec![T::default(); capacity].into_boxed_slice(),
            length: 0,
            capacity,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, item: T)
    where
        T: Default + Clone
    {
        if self.length >= self.capacity {
            self.capacity *= 2;
            let mut new_vec = vec![T::default(); self.capacity].into_boxed_slice();
            new_vec[..self.length].clone_from_slice(&self.data[..self.length]);
            self.data = new_vec;
        }
        self.data[self.length] = item;
        self.length += 1;
    }

    pub fn iter<'a>(&'a self) -> VectorIterator<'a, T> {
        VectorIterator {
            vector: &self,
            index: 0,
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> VectorIterMut<'a, T> {
        VectorIterMut {
            vector: self,
            index: 0,
        }
    }
}

impl<T: Default> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = VectorIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        VectorIntoIter {
            vector: self,
            index: 0,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
        .entries(&self.data[..self.length])
        .finish()
    }
}

pub struct VectorIntoIter<T> {
    vector: Vector<T>,
    index: usize,
}

impl<T: Default> Iterator for VectorIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vector.length {
            let item = std::mem::take(&mut self.vector.data[self.index]);
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VectorIterator<'a, T> {
    vector: &'a Vector<T>,
    index: usize,
}

impl<'a, T> Iterator for VectorIterator<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vector.length {
            let item = &self.vector.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VectorIterMut<'a, T> {
    vector: &'a mut Vector<T>,
    index: usize,
}

impl<'a, T> Iterator for VectorIterMut<'a, T> {
    type Item = &'a mut T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vector.length {
            //how?
            let item = unsafe { &mut *self.vector.data.as_mut_ptr().add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}