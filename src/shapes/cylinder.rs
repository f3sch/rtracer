use crate::*;
use std::f64::{INFINITY, NEG_INFINITY};
use uuid::Uuid;

/// Cube.
#[derive(Debug)]
pub struct Cylinder {
    /// Unique id.
    uuid: Uuid,

    /// Transformation matrix
    transform: Transformation,

    /// The material of a sphere
    material: Material,

    /// Minimum of cylinder.
    minimum: f64,

    /// Maximum of cylinder.
    maximum: f64,

    /// Is the cylinder closed.
    closed: bool,
}

impl Cylinder {
    /// Create a new sphere.
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            transform: Transformation::new(),
            material: Material::default(),
            minimum: NEG_INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }

    pub fn set_color(&mut self, color: RGB) {
        self.material.color = color;
    }

    pub fn set_cuts(&mut self, min: f64, max: f64) {
        self.minimum = min;
        self.maximum = max;
    }

    pub fn set_closed(&mut self, is_closed: bool) {
        self.closed = is_closed;
    }

    /// checks to see if the intersection at `t` is within a radius
    /// of 1 (the radius of your cylinders) from the y axis.
    fn check_cap(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        (x.powi(2) + z.powi(2)) <= 1.0
    }

    fn intersect_caps(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();

        // caps only matter if the cylinder is closed, and might possibly be
        // intersected by the ray.
        if !self.closed || float_eq(ray.direction.y, 0.0) {
            return None;
        }

        // check for an intersection with the lower end cap by intersecting
        // the ray with the plane at y=cyl.minimum
        let t = (self.minimum - ray.origin.y) / ray.direction.y;
        if Self::check_cap(ray, t) {
            xs.push(Intersection::new(t, self));
        }

        // check for an intersection with the upper end cap by intersecting
        // the ray with the plane at y=cyl.maximum
        let t = (self.maximum - ray.origin.y) / ray.direction.y;
        if Self::check_cap(ray, t) {
            xs.push(Intersection::new(t, self));
        }

        if xs.is_empty() {
            None
        } else {
            Some(xs)
        }
    }
}

impl Shape for Cylinder {
    fn id(&self) -> Uuid {
        self.uuid
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    fn get_transform(&self) -> Transformation {
        self.transform
    }

    fn set_transform(&mut self, t: Transformation) {
        self.transform = t;
    }

    fn local_intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        if float_eq(a, 0.0) {
            return self.intersect_caps(ray);
        }

        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;

        if disc < 0.0 {
            return None;
        }

        let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
        let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
        if t0 > t1 {
            (t0, t1) = (t1, t0);
        }

        let mut xs: Vec<Intersection> = Vec::new();

        let y0 = ray.origin.y + t0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection::new(t0, self));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection::new(t1, self));
        }

        if let Some(cxs) = self.intersect_caps(ray) {
            for i in cxs {
                xs.push(i);
            }
        }

        if xs.len() == 0 {
            None
        } else {
            Some(xs)
        }
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        let dist = point.x.powi(2) + point.z.powi(2);

        if dist < 1.0 && point.y >= self.maximum - EPSILON {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y <= self.minimum + EPSILON {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            Vector::new(point.x, 0.0, point.z)
        }
    }
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn miss_cylinder() {
        let cyl = Cylinder::new();
        let data = vec![
            (Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0)),
        ];
        for rec in data {
            let direction = rec.1.normalize();
            let r = Ray::new(rec.0, direction);
            let xs = cyl.local_intersect(&r);

            assert!(xs.is_none());
        }
    }

    #[test]
    fn strike_cylinder() {
        let cyl = Cylinder::new();
        let data = vec![
            (
                Point::new(1.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];
        for rec in data {
            let direction = rec.1.normalize();
            let r = Ray::new(rec.0, direction);
            let xs = cyl.local_intersect(&r);
            assert!(xs.is_some());
            let xs = xs.unwrap();

            assert_eq!(xs.len(), 2);
            assert!(float_eq(xs[0].t, rec.2));
            assert!(float_eq(xs[1].t, rec.3));
        }
    }

    #[test]
    fn default_cylinder() {
        let cyl = Cylinder::new();

        assert_eq!(cyl.minimum, NEG_INFINITY);
        assert_eq!(cyl.maximum, INFINITY);
    }

    #[test]
    fn normal_cylinder() {
        let cyl = Cylinder::new();
        let data = vec![
            (Point::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(0.0, 5.0, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(0.0, -2.0, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(-1.0, 1.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for rec in data {
            let n = cyl.local_normal_at(rec.0);

            assert_eq!(n, rec.1);
        }
    }

    #[test]
    fn constrain_cylinder() {
        let mut cyl = Cylinder::new();
        cyl.set_cuts(1.0, 2.0);
        let data = vec![
            (Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
            (Point::new(0.0, 3.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
        ];
        for rec in data {
            let direction = rec.1.normalize();
            let r = Ray::new(rec.0, direction);
            let xs = cyl.local_intersect(&r);
            if rec.2 == 0 {
                assert!(xs.is_none());
            } else {
                assert!(xs.is_some());
                let xs = xs.unwrap();
                assert_eq!(xs.len(), rec.2);
            }
        }
    }

    #[test]
    fn closed_cylinder() {
        let cyl = Cylinder::new();

        assert!(!cyl.closed);
    }

    #[test]
    fn caps_cylinder() {
        let mut cyl = Cylinder::new();
        cyl.set_cuts(1.0, 2.0);
        cyl.set_closed(true);
        let data = vec![
            (Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0), 2),
            (Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0), 2),
            (Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0), 2),
            (Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0), 2),
        ];
        for rec in data {
            let direction = rec.1.normalize();
            let r = Ray::new(rec.0, direction);
            let xs = cyl.local_intersect(&r);
            assert!(xs.is_some());
            let xs = xs.unwrap();

            assert_eq!(xs.len(), rec.2);
        }
    }
    #[test]
    pub fn normal_caps_cylinder() {
        let mut c = Cylinder::new();
        c.minimum = 1.0;
        c.maximum = 2.0;
        c.closed = true;
        let data = vec![
            (Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.5, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.0, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.0, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.5, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 2.0, 0.5), Vector::new(0.0, 1.0, 0.0)),
        ];

        for rec in data {
            let n = c.local_normal_at(rec.0);
            assert_eq!(rec.1, n);
        }
    }
}
