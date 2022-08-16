use crate::*;
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
    fn get_material(&self) -> &Material;

    /// Return the material of a shape
    fn get_material_mut(&mut self) -> &mut Material;

    /// Set the material of a shape
    fn set_material(&mut self, m: Material);

    /// Every shape has an internal transformation matrix
    fn get_transform(&self) -> Transformation;

    /// Set the transformation of any shape
    fn set_transform(&mut self, t: Transformation);

    /// Get parent id of an `object`
    fn parent_id(&self) -> Option<Uuid>;

    /// Set parent id of an `object`
    fn set_parent_id(&mut self, id: Uuid);

    /// If the object is a container then get child with `id`.
    fn get_object_by_id(&self, _id: Uuid) -> Option<&dyn Shape> {
        None
    }

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
    fn normal_at(&self, point: Point, w: Option<&World>) -> Vector {
        match w {
            Some(w) => {
                let local_point = self.world_to_object(point, w);
                let local_normal = self.local_normal_at(local_point);
                self.normal_to_world(local_normal, w)
            }
            None => {
                let inv = self
                    .get_transform()
                    .init()
                    .inverse(4)
                    .expect("Transform should have an inverse!");
                let local_point = inv * point;
                let local_normal = self.local_normal_at(local_point);
                (inv.transpose() * local_normal).normalize()
            }
        }
    }

    fn world_to_object(&self, point: Point, w: &World) -> Point {
        let object_point = match self.parent_id() {
            Some(id) => {
                let parent = w.get_object_by_id(id).expect("Shape not found!");
                parent.world_to_object(point, w)
            }
            None => point,
        };

        self.get_transform().init().inverse(4).unwrap() * object_point
    }

    /// Compute the local normal.
    fn local_normal_at(&self, point: Point) -> Vector;

    /// Calculate the normal in world space.
    fn normal_to_world(&self, normal: Vector, w: &World) -> Vector {
        let world_normal =
            (self.get_transform().init().inverse(4).unwrap().transpose() * normal).normalize();

        match self.parent_id() {
            Some(id) => {
                let parent = w.get_object_by_id(id).expect("Shape not found!");
                parent.normal_to_world(world_normal, w)
            }
            None => world_normal,
        }
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

/// export all known shapes
pub mod sphere;
pub use sphere::Sphere;
pub mod plane;
pub use plane::Plane;
pub mod cube;
pub use cube::Cube;
pub mod cylinder;
pub use cylinder::Cylinder;
pub mod cone;
pub use cone::Cone;
pub mod group;
pub use group::Group;
