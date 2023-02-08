use std::ops::{Index, IndexMut};

pub struct Matrix<T> {
    elem: Vec<T>,
    width: usize,
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            elem: vec![T::default(); w * h],
            width: w,
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let line_start = self.width * index;
        &self.elem[line_start..line_start + self.width]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let line_start = self.width * index;
        &mut self.elem[line_start..line_start + self.width]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut m = Matrix::new(3, 5);
        m.elem[4] = 8;
        // need index trait to retrieve value with [][]
        assert_eq!(m[1][1], 8);

        // need index_mut trait to assign value
        m[2][0] = 10;
        assert_eq!(m.elem[6], 10);
    }
}
