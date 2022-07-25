use crate::{intersection::Intersection, Transformation, Ray};
use std::fmt::Debug;
use uuid::Uuid;

/// Common trait among all shapes.
pub trait Shape: Debug {
    /// Every shape has a unique id in the world.
    fn id(&self) -> Uuid;

    /// check for equaltiy
    fn eq(&self, other: &dyn Shape) -> bool {
        self.id() == other.id()
    }

    /// Every shape has an internal transformation matrix
    fn get_transform(&self) -> Transformation;

    /// Set the transformation of any shape
    fn set_transform(&mut self, t: Transformation);

    /// A ray _can_ intersect a shape.
    /// This returns a collection of unit time(s) 't',
    /// when the ray intersects the shape.
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>>;
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

/// export all known shapes
mod sphere;
pub use sphere::Sphere;
