use std::cmp::Ordering;

pub const EPSILON: f64 = 0.0001;

#[inline(always)]
pub fn float_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[inline(always)]
pub fn float_cmp(a: f64, b: f64) -> Ordering {
    if float_eq(a, b) {
        Ordering::Equal
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[macro_export]
macro_rules! add_object {
    ($w:expr, $obj:expr) => {
        $w.add_object(Box::new($obj))
    };
}

#[macro_export]
macro_rules! set_pattern {
    ($obj:expr, $p:expr) => {
        $obj.get_material_mut().pattern = Some(Box::new($p))
    };
}

mod point;
pub use crate::point::Point;

mod vector;
pub use crate::vector::Vector;

mod color;
pub use crate::color::RGB;
pub use crate::color::{BLACK, BLUE, GREEN, RED, WHITE};

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
pub use crate::shapes::Cone;
pub use crate::shapes::Cube;
pub use crate::shapes::Cylinder;
pub use crate::shapes::Plane;
pub use crate::shapes::Shape;
pub use crate::shapes::Sphere;

mod intersection;
pub use crate::intersection::Intersection;

mod light;
pub use crate::light::PointLight;

mod material;
pub use crate::material::Material;

mod world;
pub use crate::world::World;

mod computations;
pub use crate::computations::Computation;

mod camera;
pub use crate::camera::Camera;

pub mod pattern;
pub use crate::pattern::Checkers;
pub use crate::pattern::Gradient;
pub use crate::pattern::Pattern;
pub use crate::pattern::Ring;
pub use crate::pattern::Stripes;
