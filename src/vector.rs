use float_eq::float_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// The Vector in a left-coordinate system.
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    /// Distance from origin along the X axis.
    pub x: f64,
    /// Distance from origin along the Y axis.
    pub y: f64,
    /// Distance from origin along the Z axis.
    pub z: f64,
}

impl Vector {
    /// Creates a Vector in space.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Calculate the Length/Magnitude of a Vector.
    pub fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    /// Calculate the dot product of two Vectors
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculate the cross product of two Vectors
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        float_eq!(self.x, other.x, abs <= 0.00001)
            && float_eq!(self.y, other.y, abs <= 0.00001)
            && float_eq!(self.z, other.z, abs <= 0.00001)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
        assert_eq!(3.0, v.z);
    }

    #[test]
    fn add_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        let c = a + b;

        assert_eq!(c.x, 5.0);
        assert_eq!(c.y, 7.0);
        assert_eq!(c.z, 9.0);
    }

    #[test]
    fn sub_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        let c = b - a;

        assert_eq!(c.x, 3.0);
        assert_eq!(c.y, 3.0);
        assert_eq!(c.z, 3.0);
    }

    #[test]
    fn neg_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = -a;

        assert_eq!(b.x, -1.0);
        assert_eq!(b.y, -2.0);
        assert_eq!(b.z, -3.0);
    }

    #[test]
    fn scal_mult_vector() {
        let a = Vector::new(-2.0, 3.0, -4.0);
        let s = 3.5;
        let b = a * s;

        assert_eq!(b.x, -7.0);
        assert_eq!(b.y, 10.5);
        assert_eq!(b.z, -14.0);
    }

    #[test]
    fn scal_div_vector() {
        let a = Vector::new(-2.0, 3.0, -4.0);
        let s = 2.0;
        let b = a / s;

        assert_eq!(b.x, -1.0);
        assert_eq!(b.y, 1.5);
        assert_eq!(b.z, -2.0);
    }

    #[test]
    fn mag_x_vector() {
        let v = Vector::new(1.0, 0.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn mag_y_vector() {
        let v = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn mag_z_vector() {
        let v = Vector::new(0.0, 0.0, 1.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn mag_pos_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.magnitude(), 14_f64.sqrt());
    }

    #[test]
    fn mag_neg_vector() {
        let v = Vector::new(-1.0, -2.0, -3.0);

        assert_eq!(v.magnitude(), 14_f64.sqrt());
    }

    #[test]
    fn norm_gives_1_vector() {
        let v = Vector::new(4.0, 0.0, 0.0);

        assert_eq!(v.normalize(), Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn norm_approx_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.normalize(), Vector::new(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn norm_magnitude_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn dot_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(a.cross(b), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), Vector::new(1.0, -2.0, 1.0));
    }
}
