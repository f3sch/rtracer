use crate::{float_eq, Vector};
use std::{
    fmt,
    ops::{Add, Neg, Sub},
};

/// The Point in a left-coordinate system from the origin.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    /// Distance from origin along the X axis.
    pub x: f64,
    /// Distance from origin along the Y axis.
    pub y: f64,
    /// Distance from origin along the Z axis.
    pub z: f64,
}

impl Point {
    /// Creates a Point in space.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Add<Vector> for Point {
    type Output = Self;
    fn add(self, other: Vector) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, other: Self) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x: {0:>10} y: {1:>10} z: {2:>10}",
            format!("{0:.5}", self.x),
            format!("{0:.5}", self.y),
            format!("{0:.5}", self.z)
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_point() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(1.0, p.x);
        assert_eq!(2.0, p.y);
        assert_eq!(3.0, p.z);
    }

    #[test]
    fn add_points() {
        let a = Point::new(1.0, 2.0, 3.0);
        let b = Point::new(4.0, 5.0, 6.0);
        let c = a + b;

        assert_eq!(c.x, 5.0);
        assert_eq!(c.y, 7.0);
        assert_eq!(c.z, 9.0);
    }

    #[test]
    fn sub_points() {
        let a = Point::new(1.0, 2.0, 3.0);
        let b = Point::new(4.0, 5.0, 6.0);
        let c = b - a;

        assert_eq!(c.x, 3.0);
        assert_eq!(c.y, 3.0);
        assert_eq!(c.z, 3.0);
    }

    #[test]
    fn neg_points() {
        let a = Point::new(1.0, 2.0, 3.0);
        let b = -a;

        assert_eq!(b.x, -1.0);
        assert_eq!(b.y, -2.0);
        assert_eq!(b.z, -3.0);
    }
}
