use crate::*;
use std::f64::{INFINITY, NEG_INFINITY};
use uuid::Uuid;

/// Cone.
#[derive(Debug)]
pub struct Cone {
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

impl Cone {
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
        let y = ray.origin.y + t * ray.direction.y;

        x.powi(2) + z.powi(2) <= y.abs()
    }

    fn intersect_caps(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();

        // caps only matter if the cone is closed, and might possibly be
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

impl Shape for Cone {
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
        let mut xs: Vec<Intersection> = Vec::new();

        let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);

        let b = 2.0 * ray.origin.x * ray.direction.x - 2.0 * ray.origin.y * ray.direction.y
            + 2.0 * ray.origin.z * ray.direction.z;

        let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

        if float_eq(a, 0.0) && float_eq(b, 0.0) {
            return None;
        }

        if float_eq(a, 0.0) && b != 0.0 {
            xs.push(Intersection::new(-c / (2.0 * b), self));
        }

        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }

        let mut t = (
            (-b - disc.sqrt()) / (2.0 * a),
            (-b + disc.sqrt()) / (2.0 * a),
        );

        if t.0 > t.1 {
            t = (t.1, t.0);
        }

        let y0 = ray.origin.y + t.0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection::new(t.0, self));
        }

        let y1 = ray.origin.y + t.1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection::new(t.1, self))
        }

        if let Some(cxs) = self.intersect_caps(ray) {
            for i in cxs {
                xs.push(i)
            }
        }

        if xs.is_empty() {
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
        } else if point.y > 0.0 {
            let y = -((point.x.powi(2) + point.z.powi(2)).sqrt());
            Vector::new(point.x, y, point.z)
        } else {
            let y = (point.x.powi(2) + point.z.powi(2)).sqrt();
            Vector::new(point.x, y, point.z)
        }
    }
}

impl PartialEq for Cone {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Default for Cone {
    fn default() -> Self {
        Cone::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_cone() {
        let c = Cone::new();
        let data = vec![
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                Point::new(1.0, 1.0, -5.0),
                Vector::new(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
            ),
        ];
        for rec in data {
            let direction = rec.1;
            let r = Ray::new(rec.0, direction.normalize());
            let xs = c.local_intersect(&r).unwrap();

            assert_eq!(2, xs.len());
            assert!(float_eq(xs[0].t, rec.2));
            assert!(float_eq(xs[1].t, rec.3));
        }
    }

    #[test]
    fn parallel_intersect_cone() {
        let c = Cone::new();
        let direction = Vector::new(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -1.0), direction);
        let xs = c.local_intersect(&r).unwrap();

        assert_eq!(1, xs.len());
        assert!(float_eq(xs[0].t, 0.35355));
    }

    #[test]
    fn caps_cone() {
        let mut c = Cone::new();
        c.minimum = -0.5;
        c.maximum = 0.5;
        c.closed = true;
        let data = vec![
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0), 0),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0), 4),
        ];
        for rec in data {
            let direction = rec.1;
            let r = Ray::new(rec.0, direction.normalize());
            match c.local_intersect(&r) {
                Some(xs) => assert_eq!(rec.2, xs.len()),
                None => assert_eq!(rec.2, 0),
            }
        }
    }

    #[test]
    fn normal_cone() {
        let cone = Cone::new();
        let data = vec![
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
            (
                Point::new(1.0, 1.0, 1.0),
                Vector::new(1.0, -2_f64.sqrt(), 1.0),
            ),
            (Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0)),
        ];
        for rec in data {
            let n = cone.local_normal_at(rec.0);
            assert_eq!(n, rec.1);
        }
    }
}
