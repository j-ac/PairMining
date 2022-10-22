/// An matrix for data where [x,y] is not meaninfully distinct from [y,x]. x must equal y (ie half of a square matrix)
/// Does not allocate memory for the half of the matrix going unused, so works for applications where waste is unacceptable
/// Insertions/Queries will be reordered so the lesser is done first, for this reason its data must implement Ord.
use std::ops::Index;
struct TriangleMatrix{
    array: Vec<Vec<usize>>,
    size: usize,
}

impl TriangleMatrix{
    fn new(size: usize) -> Self{
        let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(size);
        let mut col_num: usize = 0;
        for column in 0 .. matrix.len() { //Make each column one smaller than the previous
            matrix[column] = Vec::with_capacity(size - col_num);
            col_num += 1;
        }

    
        Self { array: Vec::new(), size: 0 } //placeholder
    }

    

    /// Always increments at the index of [lesser_value, greater_value]
    fn increment(&mut self, x: usize, y: usize){
        let (l,r) = sort(x,y);
        assert!(l <= self.array.capacity());
        assert!(r <= self.array.get(l).unwrap().capacity());
        let val = self.array.get_mut(l).unwrap().get(r).unwrap() + 1;
        self.array.get_mut(l).unwrap().insert(r, val);
        
    }

    
    
}

impl Index<(usize, usize)> for TriangleMatrix {
    type Output = usize;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x,y) = sort(index.0, index.1);

        self.array.get(x).unwrap().get(y).unwrap()

        
    }
}

// For ensuring values are ordered (least, greatest)
fn sort(x: usize, y: usize) -> (usize, usize){
    if y > x {
        return (x, y)
    }

    (y,x)
    
}