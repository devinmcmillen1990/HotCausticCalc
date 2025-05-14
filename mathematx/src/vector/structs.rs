#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Vector<T> {
    pub data: Vec<T>,
}

// TODO: provide a practical example of optimizing a trait implementation, such as a custom Clone or Hash