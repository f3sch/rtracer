use float_eq::float_eq;
use std::ops::{Index, IndexMut, Mul};

/// Matrix 4x4 implementation (rows first)
#[derive(Debug)]
pub struct Matrix {
    pub data: [[f64; 4]; 4],
}

impl Matrix {
    /// Create a new 4x4 Matrix
    pub fn new(data: [[f64; 4]; 4]) -> Self {
        Self { data }
    }
}

impl Index<usize> for Matrix {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for r in 0..4 {
            for c in 0..4 {
                if !float_eq!(self.data[r][c], other.data[r][c], abs <= 0.00001) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_matrix2() {
        let m = Matrix::new([
            [-3.0, 5.0, 0.0, 0.0],
            [1.0, -2.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn init_matrix3() {
        let m = Matrix::new([
            [-3.0, 5.0, 0.0, 0.0],
            [1.0, -2.0, -7.0, 0.0],
            [0.0, 1.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn init_matrix4() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.6, 15.5, 16.5],
        ]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn compare_eq_matrix(){
        let a = Matrix::new([
            [1.0,2.0,3.0,4.0],
            [5.0,6.0,7.0,8.0],
            [9.0,8.0,7.0,6.0],
            [5.0,4.0,3.0,2.0],
        ]);
        let b = Matrix::new([
            [1.0,2.0,3.0,4.0],
            [5.0,6.0,7.0,8.0],
            [9.0,8.0,7.0,6.0],
            [5.0,4.0,3.0,2.0],
        ]);

        assert_eq!(a,b);
    }

    #[test]
    fn compare_neq_matrix(){
        let a = Matrix::new([
            [1.0,2.0,3.0,4.0],
            [5.0,6.0,7.0,8.0],
            [9.0,8.0,7.0,6.0],
            [5.0,4.0,3.0,2.0],
        ]);
        let b = Matrix::new([
            [2.0,3.0,4.0,5.0],
            [6.0,7.0,8.0,9.0],
            [8.0,7.0,6.0,5.0],
            [4.0,3.0,2.0,1.0],
        ]);

        assert_ne!(a,b);
    }
}
