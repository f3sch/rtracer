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
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let local_ray = ray.transform(
            self.get_transform()
                .init()
                .inverse(4)
                .expect("The transformation matrix should invertible!"),
        );
        self.local_intersect(&local_ray)
    }

    /// Perform the actual intersection of the ray.
    fn local_intersect(&self, ray: &Ray) -> Option<Vec<Intersection>>;

    /// Compute a normal at a given point for a shape.
    fn normal_at(&self, world_point: Point) -> Vector {
        let inv = self.get_transform().init().inverse(4).unwrap();
        let local_point = inv * world_point;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = inv.transpose() * local_normal;

        world_normal.normalize()
    }

    /// Compute the local normal.
    fn local_normal_at(&self, point: Point) -> Vector;
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

/// export all known shapes
pub mod sphere;
pub use sphere::Sphere;
