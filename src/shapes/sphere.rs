use crate::{shapes::Shape, Intersection, Point, Ray};
use uuid::Uuid;

/// A sphere.
#[derive(Debug)]
pub struct Sphere {
    /// Unique id.
    uuid: Uuid,
    /// Centre of the sphere.
    origin: Point,
    /// Radius of the sphere.
    radius: f64,
}

impl Sphere {
    /// Create a new sphere.
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            origin: Point::new(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }
}

impl Shape for Sphere {
    fn id(&self) -> Uuid {
        self.uuid
    }

    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let sphere_to_ray = ray.origin - self.origin;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            Some(vec![Intersection::new(t1, self), Intersection::new(t2, self)])
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Point, Ray, Vector};

    #[test]
    fn unique_sphere() {
        let s1 = Sphere::new();
        let s2 = Sphere::new();

        assert_ne!(s1.id(), s2.id());
    }

    #[test]
    fn intersect_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.is_some(), true);
        let xs = xs.unwrap();

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn intersect_tangent_sphere() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.is_some(), true);
        let xs = xs.unwrap();

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn miss_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.is_none(), true);
    }

    #[test]
    fn inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.is_some(), true);
        let xs = xs.unwrap();

        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn behind_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.is_some(), true);
        let xs = xs.unwrap();

        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn object_sphere(){
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.is_some(), true);
        let xs = xs.unwrap();

        assert_eq!(xs[0].object.eq(&s),true);
        assert_eq!(xs[1].object.eq(&s),true);
    }
}
