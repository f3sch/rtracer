use crate::{intersection::Intersection, Ray};
use std::fmt::Debug;
use uuid::Uuid;

/// Common trait among all shapes.
pub trait Shape<S: PartialEq + Debug + Shape<S>> {
    /// Every shape has a unique id in the world.
    fn id(&self) -> Uuid;
    /// A ray _can_ intersect a shape.
    /// This returns a collection of unit time(s) 't',
    /// when the ray intersects the shape.
    fn intersect(&self, ray: &Ray) -> Option<[Intersection<S>; 2]>;
}

mod sphere;

pub use sphere::Sphere;
