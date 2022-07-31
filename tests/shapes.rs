use std::f64::consts::PI;

use rtracer::*;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default)]
struct TestShape {
    uuid: Uuid,
    material: Material,
    transform: Transformation,
}

static mut SAVE_RAY: Ray = Ray {
    origin: Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
    direction: Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
};

impl Shape for TestShape {
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

    fn local_intersect(&self, _ray: &Ray) -> Option<Vec<Intersection>> {
        None
    }

    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        unsafe {
            SAVE_RAY = ray.transform(
                self.get_transform()
                    .init()
                    .inverse(4)
                    .expect("The transformation matrix should invertible!"),
            );
        }
        None
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        point - Point::new(0.0, 0.0, 0.0)
    }
}

#[test]
fn default_transformation() {
    let s = TestShape::default();

    assert_eq!(s.transform.init(), IDENTITY);
}

#[test]
fn set_transform() {
    let mut s = TestShape::default();
    s.set_transform(Transformation::new().translation(2.0, 3.0, 4.0));

    assert_eq!(
        s.transform,
        Transformation::new().translation(2.0, 3.0, 4.0)
    );
}

#[test]
fn default_material() {
    let s = TestShape::default();
    let m = s.get_material();

    assert_eq!(m, Material::default());
}

#[test]
fn set_material() {
    let mut s = TestShape::default();
    let mut m = s.get_material();
    m.ambient = 1.0;
    s.set_material(m);

    assert_eq!(s.material, m);
}

#[test]
fn intersect_scaled_shape_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let mut s = TestShape::default();
    s.set_transform(Transformation::new().scaling(2.0, 2.0, 2.0));
    let _xs = s.intersect(&r);

    unsafe {
        assert_eq!(SAVE_RAY.origin, Point::new(0.0, 0.0, -2.5));
        assert_eq!(SAVE_RAY.direction, Vector::new(0.0, 0.0, 0.5));
    }
}

#[test]
fn intersect_translated_shape_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let mut s = TestShape::default();
    s.set_transform(Transformation::new().translation(5.0, 0.0, 0.0));
    let _xs = s.intersect(&r);

    unsafe {
        assert_eq!(SAVE_RAY.origin, Point::new(-5.0, 0.0, -5.0));
        assert_eq!(SAVE_RAY.direction, Vector::new(0.0, 0.0, 1.0));
    }
}

#[test]
fn normal_translated_shape() {
    let mut s = TestShape::default();
    s.set_transform(Transformation::new().translation(0.0, 1.0, 0.0));
    let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

    assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
}

#[test]
fn normal_transformed_shape() {
    let mut s = TestShape::default();
    let m = Transformation::new()
        .rotate_z(PI / 5.0)
        .scaling(1.0, 0.5, 1.0);
    s.set_transform(m);
    let n = s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt() / 2.0)));

    assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
}
