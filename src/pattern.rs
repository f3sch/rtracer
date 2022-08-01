use crate::{Point, Shape, Transformation, RGB};
use std::fmt::Debug;
use uuid::Uuid;

/// This traits describes all patterns.
pub trait Pattern: Debug {
    /// Used for comparing patterns.
    fn id(&self) -> Uuid;

    /// Call pattern specific function, calculate pattern_point.
    fn pattern_at_shape(&self, shape: &dyn Shape, point: Point) -> RGB {
        let object_point = shape
            .get_transform()
            .init()
            .inverse(4)
            .expect("Object transform should be invertible")
            * point;
        let pattern_point = self
            .get_transform()
            .init()
            .inverse(4)
            .expect("Pattern transform should be invertible")
            * object_point;

        self.pattern_at(pattern_point)
    }

    /// Each Pattern needs to implement this/
    fn pattern_at(&self, point: Point) -> RGB;

    /// Return the transformation matrix.
    fn get_transform(&self) -> Transformation;

    /// Set the transformation matrix.
    fn set_transform(&mut self, t: Transformation);
}

impl PartialEq for Box<dyn Pattern> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

pub mod stripes;
pub use stripes::Stripes;
