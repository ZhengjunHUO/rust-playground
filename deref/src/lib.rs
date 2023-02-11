use std::ops::{Deref, DerefMut};

pub struct Pool<T> {
    content: Vec<T>,
    current: usize,
}

impl<T> Pool<T> {
    pub fn new(content: Vec<T>, current: usize) -> Pool<T> {
        Pool { content, current }
    }
}

impl<T> Deref for Pool<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.content[self.current]
    }
}

impl<T> DerefMut for Pool<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.content[self.current]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool() {
        let mut pool = Pool {
            content: vec![10, 20, 30, 40],
            current: 1,
        };
        assert_eq!(*pool, 20);
        *pool = 88;
        assert_eq!(pool.content, [10, 88, 30, 40]);
    }
}
