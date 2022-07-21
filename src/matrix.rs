use crate::{Point, Vector};
use float_eq::float_eq;
use std::ops::{Index, IndexMut, Mul};

/// Matrix 4x4 implementation (rows first)
#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    pub data: [[f64; 4]; 4],
}

pub const IDENTITY: Matrix = Matrix {
    data: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

impl Matrix {
    /// Create a new 4x4 Matrix
    pub fn new(data: [[f64; 4]; 4]) -> Self {
        Self { data }
    }

    pub fn transpose(&self) -> Self {
        let mut trans = IDENTITY;
        for r in 0..4 {
            for c in 0..4 {
                trans[c][r] = self[r][c];
            }
        }
        trans
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [[0.0; 4]; 4];

        for r in 0..4 {
            for c in 0..4 {
                data[r][c] = self[r][0] * rhs[0][c]
                    + self[r][1] * rhs[1][c]
                    + self[r][2] * rhs[2][c]
                    + self[r][3] * rhs[3][c];
            }
        }
        Self { data }
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: (self[0][0] * rhs.x)
                + (self[0][1] * rhs.y)
                + (self[0][2] * rhs.z)
                + (self[0][3] * 1.0),
            y: (self[1][0] * rhs.x)
                + (self[1][1] * rhs.y)
                + (self[1][2] * rhs.z)
                + (self[1][3] * 1.0),
            z: (self[2][0] * rhs.x)
                + (self[2][1] * rhs.y)
                + (self[2][2] * rhs.z)
                + (self[2][3] * 1.0),
        }
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: (self[0][0] * rhs.x)
                + (self[0][1] * rhs.y)
                + (self[0][2] * rhs.z)
                + (self[0][3] * 1.0),
            y: (self[1][0] * rhs.x)
                + (self[1][1] * rhs.y)
                + (self[1][2] * rhs.z)
                + (self[1][3] * 1.0),
            z: (self[2][0] * rhs.x)
                + (self[2][1] * rhs.y)
                + (self[2][2] * rhs.z)
                + (self[2][3] * 1.0),
        }
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
    fn eq(&self, rhs: &Self) -> bool {
        for r in 0..4 {
            for c in 0..4 {
                if !float_eq!(self.data[r][c], rhs.data[r][c], abs <= 0.00001) {
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
    fn compare_eq_matrix() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(a, b);
    }

    #[test]
    fn compare_neq_matrix() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_ne!(a, b);
    }

    #[test]
    fn mul_matrix() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let c = Matrix::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(a * b, c);
    }

    #[test]
    fn mul_point_matrix() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Point::new(1.0, 2.0, 3.0);
        let c = Point::new(18.0, 24.0, 33.0);

        assert_eq!(a * b, c);
    }

    #[test]
    fn mul_vector_matrix() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Vector::new(1.0, 2.0, 3.0);
        let c = Vector::new(18.0, 24.0, 33.0);

        assert_eq!(a * b, c);
    }

    #[test]
    fn mul_id_matrix() {
        let a = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        assert_eq!(IDENTITY * a, a);
    }

    #[test]
    fn transpose_matrix() {
        let a = Matrix::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let t = Matrix::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(a.transpose(), t);

    }

    #[test]
    fn transpose_id_matrix(){
        assert_eq!(IDENTITY.transpose(), IDENTITY);
    }
}
