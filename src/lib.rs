use std::cmp::Ordering;

pub const EPSILON: f64 = 0.0001;

pub fn float_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn float_cmp(a: f64, b: f64) -> Ordering {
    if float_eq(a, b) {
        Ordering::Equal
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

mod point;
pub use crate::point::Point;

mod vector;
pub use crate::vector::Vector;

mod color;
pub use crate::color::RGB;

mod canvas;
pub use crate::canvas::Canvas;

mod matrix;
pub use crate::matrix::Matrix;
pub use crate::matrix::IDENTITY;

mod transformations;
pub use crate::transformations::Transformation;

mod ray;
pub use crate::ray::Ray;

pub mod shapes;
pub use crate::shapes::Shape;
pub use crate::shapes::Sphere;

mod intersection;
pub use crate::intersection::Intersection;
