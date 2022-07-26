use crate::{Point, Vector, shapes::Shape};

/// A Computation encapsulates some pre-compute information of an intersection and an object.
pub struct Computation<'a>{
    /// Distance from the origin of a Ray to the intersection.
    pub t: f64,

    /// The object intersected by a Ray.
    pub object: &'a dyn Shape,

    /// Point in world space where the Intersection occured.
    pub point: Point,

    /// Eye Vector of the surface of the object.
    pub eyev: Vector,

    /// Normal Vector of the surface of the object.
    pub normalv: Vector,

    /// Is the intersection inside of the shape?
    pub inside: bool,
}
