use crate::{shapes::Shape, Intersection, Point, Ray, Transformation};
use uuid::Uuid;

/// A sphere.
#[derive(Debug)]
pub struct Sphere {
    /// Unique id.
    uuid: Uuid,

    /// Transformation matrix
    transform: Transformation,
}

impl Sphere {
    /// Create a new sphere.
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            transform: Transformation::new(),
        }
    }
}

impl Shape for Sphere {
    fn id(&self) -> Uuid {
        self.uuid
    }

    fn get_transform(&self) -> Transformation {
        self.transform
    }

    fn set_transform(&mut self, t: Transformation) {
        self.transform = t;
    }

    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let inv = match self.transform.init().inverse(4) {
            None => return None,
            Some(inv) => inv,
        };
        let ray_t = ray.transform(inv);
        let sphere_to_ray = ray_t.origin - Point::new(0.0, 0.0, 0.0);
        let a = ray_t.direction.dot(ray_t.direction);
        let b = 2.0 * ray_t.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        Some(vec![
            Intersection::new(t1, self),
            Intersection::new(t2, self),
        ])
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
    use crate::{Point, Ray, Transformation, Vector, IDENTITY};

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

        assert_eq!(xs.len(), 2);
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

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn object_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.is_some(), true);
        let xs = xs.unwrap();

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object.eq(&s), true);
        assert_eq!(xs[1].object.eq(&s), true);
    }

    #[test]
    fn get_transform_sphere() {
        let s = Sphere::new();

        assert_eq!(s.transform.init(), IDENTITY);
    }

    #[test]
    fn set_transform_sphere() {
        let mut s = Sphere::new();
        let t = Transformation::translation(2.0, 3.0, 4.0);
        s.set_transform(t);

        assert_eq!(s.transform.init(), t.init());
    }

    #[test]
    fn intersect_scaled_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Transformation::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.is_some(), true);
        let xs = xs.unwrap();

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersect_translated_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Transformation::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.is_none(), true);
    }
}
