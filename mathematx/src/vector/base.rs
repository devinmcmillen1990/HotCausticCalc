// TODO: Move into /sequential/


use super::structs::Vector;
use std::ops;

impl<T> Vector<T> {
    // Initialize empty vector
    pub fn new() -> Self {
        Vector { data: Vec::new() }
    }

    // Initialize vector with data
    pub fn new_with(data: Vec<T>) -> Self {
        Vector { data }
    }

    /// Return Length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Gets the value at the given index.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

/// Enable [] read notation
impl<T> ops::Index<usize> for Vector<T> {
    type Output = T;

    /// Return indexed. panic if out of bounds
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

/// Enable [] write notation
impl<T> ops::IndexMut<usize> for Vector<T> {
    /// Return mutable indexed. panic if out of bounds
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Enable .clone()
// impl<T: Clone> Clone for Vector<T> {
//     fn clone(&self) -> Self {
//         Self {
//             data: self.data.clone(),
//         }
//     }
// }
