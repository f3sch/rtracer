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

mod shapes;
pub use crate::shapes::Sphere;

mod intersection;
pub use crate::intersection::Intersection;
