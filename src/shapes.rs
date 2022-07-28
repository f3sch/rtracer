use crate::{intersection::Intersection, Material, Point, Ray, Transformation, Vector};
use std::fmt::Debug;
use uuid::Uuid;

/// Common trait among all shapes.
pub trait Shape: 'static + Debug {
    /// Every shape has a unique id in the world.
    fn id(&self) -> Uuid;

    /// check for equality
    fn eq(&self, other: &dyn Shape) -> bool {
        self.id() == other.id()
    }

    /// Return the material of a shape
    fn get_material(&self) -> Material;

    /// Return the material of a shape
    fn get_material_mut(&mut self) -> &mut Material;

    /// Set the material of a shape
    fn set_material(&mut self, m: Material);

    /// Every shape has an internal transformation matrix
    fn get_transform(&self) -> Transformation;

    /// Set the transformation of any shape
    fn set_transform(&mut self, t: Transformation);

    /// A ray _can_ intersect a shape.
    /// This returns a collection of unit time(s) 't',
    /// when the ray intersects the shape.
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>>;

    /// Compute a normal at a given point for a shape.
    fn normal_at(&self, world_point: Point) -> Vector;
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

/// export all known shapes
pub mod sphere;
pub use sphere::Sphere;
