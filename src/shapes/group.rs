use crate::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct Group {
    id: Uuid,
    parent_id: Option<Uuid>,
    pub transform: Transformation,
    pub material: Material,
    pub objects: Vec<Box<dyn Shape>>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            id: Uuid::new_v4(),
            parent_id: None,
            transform: Transformation::new(),
            material: Material::default(),
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, mut shape: Box<dyn Shape>) {
        shape.set_parent_id(self.id);
        self.objects.push(shape);
    }

    pub fn get_object(&self, index: usize) -> Option<&dyn Shape> {
        match self.objects.get(index) {
            Some(o) => Some(o.as_ref()),
            None => None,
        }
    }
}

impl Shape for Group {
    fn id(&self) -> Uuid {
        self.id
    }

    fn parent_id(&self) -> Option<Uuid> {
        self.parent_id
    }

    fn set_parent_id(&mut self, id: Uuid) {
        self.parent_id = Some(id);
    }

    fn get_transform(&self) -> Transformation {
        self.transform
    }

    fn set_transform(&mut self, transform: Transformation) {
        self.transform = transform;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn get_object_by_id(&self, id: Uuid) -> Option<&dyn Shape> {
        let mut shape = None;
        for s in &self.objects {
            if s.id() == id {
                shape = Some(s.as_ref());
                break;
            }
            if let Some(c) = s.get_object_by_id(id) {
                shape = Some(c);
                break;
            }
        }

        shape
    }

    fn local_intersect<'a>(&'a self, ray: &Ray) -> Option<Vec<Intersection<'a>>> {
        let mut xs: Vec<Intersection> = Vec::new();

        for o in &self.objects {
            if let Some(oxs) = o.intersect(ray) {
                for ox in oxs {
                    xs.push(ox);
                }
            }
        }

        if xs.is_empty() {
            None
        } else {
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(xs)
        }
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
        panic!("Should not be called!")
    }
}

impl Default for Group {
    fn default() -> Self {
        Group::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_group() {
        let g = Group::new();

        assert!(g.objects.is_empty());
        assert_eq!(g.transform, Transformation::new());
    }

    // Chapter 14 Groups
    // Page 195
    #[test]
    fn add_child_group() {
        let mut g = Group::new();
        let mut s = Sphere::new();
        s.set_parent_id(g.id);
        g.add_object(Box::new(s));

        assert!(!g.objects.is_empty());
        assert_eq!(g.objects[0].parent_id().unwrap(), g.id());
    }

    #[test]
    fn intersecting_empty_group() {
        let g = Group::new();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = g.local_intersect(&r);

        assert!(xs.is_none());
    }

    #[test]
    fn intersecting_none_empty_group() {
        let mut g = Group::new();

        let s1 = Sphere::new();
        let s1_id = s1.id();

        let mut s2 = Sphere::new();
        s2.set_transform(Transformation::new().translation(0.0, 0.0, -3.0));
        let s2_id = s2.id();

        let mut s3 = Sphere::new();
        s3.set_transform(Transformation::new().translation(5.0, 0.0, 0.0));

        g.add_object(Box::new(s1));
        g.add_object(Box::new(s2));
        g.add_object(Box::new(s3));

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = g.intersect(&r).unwrap();

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object.id(), s2_id);
        assert_eq!(xs[1].object.id(), s2_id);
        assert_eq!(xs[2].object.id(), s1_id);
        assert_eq!(xs[3].object.id(), s1_id);
    }

    #[test]
    fn intersecting_transformed_group() {
        let mut g = Group::new();
        g.set_transform(Transformation::new().scaling(2.0, 2.0, 2.0));

        let mut s = Sphere::new();
        s.set_transform(Transformation::new().translation(5.0, 0.0, 0.0));

        g.add_object(Box::new(s));

        let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::new(0.0, 0.0, 1.0));

        let xs = g.intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
    }
}
