use crate::{Point, Vector};
use float_eq::float_eq;
use std::{
    fmt,
    ops::{Index, IndexMut, Mul},
};

/// Matrix 4x4 implementation (rows first).
#[derive(Default, Debug, Clone, Copy)]
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
    /// Create a new 4x4 Matrix.
    pub fn new(data: [[f64; 4]; 4]) -> Self {
        Self { data }
    }

    /// Calculate the transpose of a matrix.
    pub fn transpose(&self) -> Self {
        let mut trans = Matrix::default();
        for r in 0..4 {
            for c in 0..4 {
                trans[c][r] = self[r][c];
            }
        }
        trans
    }

    /// Calculate the determinant of a matrix.
    ///
    /// 's': describes the matrix dimensions.
    fn determinant(&self, s: usize) -> f64 {
        let mut det = 0_f64;

        if s == 2 {
            det = self[0][0] * self[1][1] - self[0][1] * self[1][0];
        } else {
            for c in 0..4 {
                det += self[0][c] * self.cofactor(0, c, s - 1);
            }
        }
        det
    }

    /// Return a submatrix from the matrix with the given col,row removed.
    fn sub_matrix(&self, r: usize, c: usize) -> Self {
        let mut ret = Matrix::default();

        for (nri, ri) in [0, 1, 2, 3].iter().filter(|&&x| x != r).enumerate() {
            for (nci, ci) in [0, 1, 2, 3].iter().filter(|&&x| x != c).enumerate() {
                ret[nri][nci] = self[*ri][*ci];
            }
        }
        ret
    }

    /// Calculate the minor.
    fn minor(&self, r: usize, c: usize, s: usize) -> f64 {
        self.sub_matrix(r, c).determinant(s)
    }

    /// Calculate the cofactor.
    fn cofactor(&self, r: usize, c: usize, s: usize) -> f64 {
        let mut minor = self.minor(r, c, s);
        if (r + c) % 2 == 1 {
            minor *= -1.0
        }
        minor
    }

    /// Check if a Matrix is invertible.
    fn is_invertible(&self, s: usize) -> bool {
        !(self.determinant(s) == 0.0)
    }

    /// Calculate the inverse of a Matrix.
    /// Returns a Result<> because not every matrix is invertible.
    pub fn inverse(&self, s: usize) -> Result<Matrix, &str> {
        if !self.is_invertible(s) {
            Err("Matrix is not invertible!")
        } else {
            let mut inverse = Matrix::default();
            let d = self.determinant(s);
            for r in 0..s {
                for c in 0..s {
                    inverse[c][r] = self.cofactor(r, c, s - 1) / d;
                }
            }
            Ok(inverse)
        }
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
                + (self[0][3] * 0.0),
            y: (self[1][0] * rhs.x)
                + (self[1][1] * rhs.y)
                + (self[1][2] * rhs.z)
                + (self[1][3] * 0.0),
            z: (self[2][0] * rhs.x)
                + (self[2][1] * rhs.y)
                + (self[2][2] * rhs.z)
                + (self[2][3] * 0.0),
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

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0:>10}", format!("{0:.5}", self.data[0][0]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[0][1]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[0][2]))?;
        writeln!(f, "{0:>10}", format!("{0:.5}", self.data[0][3]))?;

        write!(f, "{0:>10}", format!("{0:.5}", self.data[1][0]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[1][1]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[1][2]))?;
        writeln!(f, "{0:>10}", format!("{0:.5}", self.data[1][3]))?;

        write!(f, "{0:>10}", format!("{0:.5}", self.data[2][0]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[2][1]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[2][2]))?;
        writeln!(f, "{0:>10}", format!("{0:.5}", self.data[2][3]))?;

        write!(f, "{0:>10}", format!("{0:.5}", self.data[3][0]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[3][1]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[3][2]))?;
        writeln!(f, "{0:>10}", format!("{0:.5}", self.data[3][3]))?;

        Ok(())
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
        let c = Vector::new(14.0, 22.0, 32.0);

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
    fn transpose_id_matrix() {
        assert_eq!(IDENTITY.transpose(), IDENTITY);
    }

    #[test]
    fn determinant_2_matrix() {
        let a = Matrix::new([
            [1.0, 5.0, 0.0, 0.0],
            [-3.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(a.determinant(2), 17.0);
    }

    #[test]
    fn determinant_3_matrix() {
        let a = Matrix::new([
            [1.0, 2.0, 6.0, 0.0],
            [-5.0, 8.0, -4.0, 0.0],
            [2.0, 6.0, 4.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(a.cofactor(0, 0, 2), 56.0);
        assert_eq!(a.cofactor(0, 1, 2), 12.0);
        assert_eq!(a.cofactor(0, 2, 2), -46.0);
        assert_eq!(a.determinant(3), -196.0);
    }

    #[test]
    fn determinant_4_matrix() {
        let a = Matrix::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_eq!(a.cofactor(0, 0, 3), 690.0);
        assert_eq!(a.cofactor(0, 1, 3), 447.0);
        assert_eq!(a.cofactor(0, 2, 3), 210.0);
        assert_eq!(a.cofactor(0, 3, 3), 51.0);
        assert_eq!(a.determinant(4), -4071.0);
    }

    #[test]
    fn sub_3_matrix() {
        let a = Matrix::new([
            [1.0, 5.0, 0.0, 0.0],
            [-3.0, 2.0, 7.0, 0.0],
            [0.0, 6.0, -3.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        let s = Matrix::new([
            [-3.0, 2.0, 0.0, 0.0],
            [0.0, 6.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(a.sub_matrix(0, 2), s);
    }

    #[test]
    fn sub_4_matrix() {
        let a = Matrix::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let s = Matrix::new([
            [-6.0, 1.0, 6.0, 0.0],
            [-8.0, 8.0, 6.0, 0.0],
            [-7.0, -1.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(a.sub_matrix(2, 1), s);
    }

    #[test]
    fn minor_matrix() {
        let a = Matrix::new([
            [3.0, 5.0, 0.0, 0.0],
            [2.0, -1.0, -7.0, 0.0],
            [6.0, -1.0, 5.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(a.minor(1, 0, 2), 25.0);
    }

    #[test]
    fn cofactor_matrix() {
        let a = Matrix::new([
            [3.0, 5.0, 0.0, 0.0],
            [2.0, -1.0, -7.0, 0.0],
            [6.0, -1.0, 5.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(a.minor(0, 0, 2), -12.0);
        assert_eq!(a.cofactor(0, 0, 2), -12.0);
        assert_eq!(a.minor(1, 0, 2), 25.0);
        assert_eq!(a.cofactor(1, 0, 2), -25.0);
    }

    #[test]
    fn is_invertible_true_matrix() {
        let a = Matrix::new([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_eq!(a.determinant(4), -2120.0);
        assert_eq!(a.is_invertible(4), true);
    }

    #[test]
    fn is_invertible_false_matrix() {
        let a = Matrix::new([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(a.determinant(4), 0.0);
        assert_eq!(a.is_invertible(4), false);
    }

    #[test]
    fn inverse_1_matrix() {
        let a = Matrix::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let res = Matrix::new([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        let b_res = a.inverse(4);
        let b = b_res.unwrap();

        assert_eq!(a.determinant(4), 532.0);
        assert_eq!(a.cofactor(2, 3, 3), -160.0);
        assert_eq!(a.cofactor(3, 2, 3), 105.0);
        assert_eq!(b_res.is_ok(), true);
        assert_eq!(-160.0 / 532.0, b[3][2]);
        assert_eq!(105.0 / 532.0, b[2][3]);
        assert_eq!(res, b);
    }

    #[test]
    fn inverse_2_matrix() {
        let a = Matrix::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let res = Matrix::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_eq!(a.inverse(4).unwrap(), res);
    }

    #[test]
    fn inverse_3_matrix() {
        let a = Matrix::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let res = Matrix::new([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_eq!(a.inverse(4).unwrap(), res);
    }

    #[test]
    fn inverse_mul_self_matrix() {
        let a = Matrix::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a * b;

        assert_eq!(c * b.inverse(4).unwrap(), a);
    }
}
