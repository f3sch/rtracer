use crate::{shapes::Shape, Intersection, Material, Point, Ray, Transformation, Vector, EPSILON};
use uuid::Uuid;

/// A xz plan.
#[derive(Debug)]
pub struct Plane {
    uuid: Uuid,
    transform: Transformation,
    material: Material,

    /// Parent id
    parent: Option<Uuid>,
}

impl Plane {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            transform: Transformation::new(),
            material: Material::default(),
            parent: None,
        }
    }
}

impl Shape for Plane {
    fn id(&self) -> Uuid {
        self.uuid
    }

    fn parent_id(&self) -> Option<Uuid> {
        self.parent
    }

    fn set_parent_id(&mut self, id: Uuid) {
        self.parent = Some(id);
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
        if ray.direction.y.abs() < EPSILON {
            return None;
        }
        let t = -ray.origin.y / ray.direction.y;
        Some(vec![Intersection { t, object: self }])
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal_const_plane() {
        let p = Plane::new();
        let n1 = p.local_normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Point::new(-5.0, 0.0, 150.0));

        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_parallel_plane() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);

        assert!(xs.is_none());
    }

    #[test]
    fn intersect_above_plane() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert!(xs.is_some());
        let xs = xs.unwrap();

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object.id(), p.id());
    }

    #[test]
    fn intersect_below_plane() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert!(xs.is_some());
        let xs = xs.unwrap();

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object.id(), p.id());
    }
}
