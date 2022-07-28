use crate::{shapes::Shape, Intersection, Material, Point, Ray, Transformation, Vector, RGB};
use uuid::Uuid;

/// A sphere.
#[derive(Debug)]
pub struct Sphere {
    /// Unique id.
    uuid: Uuid,

    /// Transformation matrix
    transform: Transformation,

    /// The material of a sphere
    material: Material,
}

impl Sphere {
    /// Create a new sphere.
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            transform: Transformation::new(),
            material: Material::default(),
        }
    }

    pub fn set_color(&mut self, color: RGB) {
        self.material.color = color;
    }
}

impl Shape for Sphere {
    fn id(&self) -> Uuid {
        self.uuid
    }

    fn get_material(&self) -> Material {
        self.material
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

    fn normal_at(&self, world_point: Point) -> Vector {
        let inv = self.transform.init().inverse(4).unwrap();
        let object_point = inv * world_point;
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        let world_normal = inv.transpose() * object_normal;
        world_normal.normalize()
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

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
        assert!(xs.is_some());
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
        assert!(xs.is_some());
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
        assert!(xs.is_none());
    }

    #[test]
    fn inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert!(xs.is_some());
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
        assert!(xs.is_some());
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
        assert!(xs.is_some());
        let xs = xs.unwrap();

        assert_eq!(xs.len(), 2);
        assert!(xs[0].object.eq(&s));
        assert!(xs[1].object.eq(&s));
    }

    #[test]
    fn get_transform_sphere() {
        let s = Sphere::new();

        assert_eq!(s.transform.init(), IDENTITY);
    }

    #[test]
    fn set_transform_sphere() {
        let mut s = Sphere::new();
        let t = Transformation::new().translation(2.0, 3.0, 4.0);
        s.set_transform(t);

        assert_eq!(s.transform.init(), t.init());
    }

    #[test]
    fn intersect_scaled_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Transformation::new().scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert!(xs.is_some());
        let xs = xs.unwrap();

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersect_translated_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Transformation::new().translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert!(xs.is_none());
    }

    #[test]
    fn normal_x_sphere() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));

        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_y_sphere() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));

        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_z_sphere() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));

        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_notaxial_sphere() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(
            n,
            Vector::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0)
        );
    }

    #[test]
    fn normal_normalize_sphere() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn normal_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Transformation::new().translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_transformed_sphere() {
        let mut s = Sphere::new();
        let t1 = Transformation::new().scaling(1.0, 0.5, 1.0);
        let t2 = Transformation::new().rotate_z(PI / 5.0);
        s.set_transform(t1 * t2);
        let n = s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt()) / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn has_material_sphere() {
        let s = Sphere::new();
        let m = s.material;

        assert_eq!(m, Material::default());
    }

    #[test]
    fn assign_material_sphere() {
        let mut s = Sphere::new();
        let mut m = s.material;
        m.ambient = 1.0;
        s.material = m;

        assert_eq!(s.material, m);
    }
}
