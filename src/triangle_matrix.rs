/// An matrix for data where [x,y] is not meaninfully distinct from [y,x]. x must equal y (ie half of a square matrix)
/// Does not allocate memory for the half of the matrix going unused, so works for applications where waste is unacceptable
/// Insertions/Queries will be reordered so the lesser is done first, for this reason its data must implement Ord.
use std::ops::Index;
pub struct TriangleMatrix {
    pub array: Vec<Vec<usize>>,
    pub size: usize,
}

impl TriangleMatrix {
    pub fn new(size: usize) -> Self {
        let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(size);
        let mut col_num: usize = 0;
        for column in 0..size {
            //Make each column one smaller than the previous
            matrix.push(Vec::with_capacity(size - col_num));

            matrix[column].resize(size - col_num, 0);

            col_num += 1;
        }

        TriangleMatrix {
            array: matrix,
            size,
        }
    }

    /// Always increments at the index of [lesser_value, greater_value]
    pub fn increment(&mut self, x: usize, y: usize) {
        let (l, r) = sort(x, y);
        //assert!(l < self.size);
        //assert!(r < self.size);
        let val = self.array.get_mut(l).unwrap().get(r).unwrap() + 1;
        self.array.get_mut(l).unwrap().insert(r, val);
    }
}

impl Index<(usize, usize)> for TriangleMatrix {
    type Output = usize;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = sort(index.0, index.1);

        self.array.get(x).unwrap().get(y).unwrap()
    }
}

// For ensuring values are ordered (least, greatest)
pub fn sort(x: usize, y: usize) -> (usize, usize) {
    if y > x {
        return (x, y);
    }

    (y, x)
}

#[cfg(test)]
mod tests {
    use crate::triangle_matrix::*;
    #[test]
    fn test_sort() {
        assert!(sort(1, 2) == (1, 2));
        assert!(sort(2, 1) == (1, 2));
    }

    #[test]
    fn test_new() {
        let tm = TriangleMatrix::new(10);
        assert!(tm.array.len() == 10);
    }

    #[test]
    fn test_index() {
        let tm = TriangleMatrix::new(40);
        assert!(tm[(4 as usize, 7 as usize)] == 0);
        assert!(tm[(39 as usize, 0 as usize)] == 0);
        assert!(tm[(38 as usize, 1 as usize)] == 0);
        assert!(tm[(5 as usize, 5 as usize)] == 0);
    }

    #[test]
    fn test_increment() {
        let mut tm = TriangleMatrix::new(10);
        tm.increment(2, 3);
        tm.increment(2, 3);
        assert!(tm[(2, 3)] == 2);
    }
}
