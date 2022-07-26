use crate::*;

/// A world holds every shape and a light source.
pub struct World {
    /// All Shapes contain in a World.
    objects: Vec<Box<dyn Shape>>,

    /// The light source.
    light: Option<Box<dyn Light>>,
}

impl World {
    /// In the beginning I created nothing.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            light: None,
        }
    }

    /// Add objects/shapes to a world.
    pub fn add_object(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }

    /// Calculate the intersection of a ray in this world.
    pub fn intersect_world(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();
        for obj in &self.objects {
            let is = obj.intersect(ray);
            if is.is_none() {
                continue;
            }
            xs.append(&mut is.unwrap());
        }

        if xs.len() == 0 {
            None
        } else {
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(xs)
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let mut w = World::new();

        w.light = Some(Box::new(PointLight::new(
            Point::new(-10.0, 10.0, -10.0),
            RGB::new(1.0, 1.0, 1.0),
        )));
        let mut s1 = Sphere::new();
        let mut m1 = Material::default();
        m1.color = RGB::new(0.8, 1.0, 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;
        s1.set_material(m1);
        w.add_object(Box::new(s1));

        let mut s2 = Sphere::new();
        let t2 = Transformation::new().scaling(0.5, 0.5, 0.5);
        s2.set_transform(t2);
        w.add_object(Box::new(s2));

        w
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_world() {
        let w = World::new();

        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }

    #[test]
    fn default_world() {
        let w = World::default();
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), RGB::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        let mut m1 = Material::default();
        m1.color = RGB::new(0.8, 1.0, 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;
        s1.set_material(m1);

        let mut s2 = Sphere::new();
        let t2 = Transformation::new().scaling(0.5, 0.5, 0.5);
        s2.set_transform(t2);

        assert_eq!(w.objects.len(), 2);
        assert_eq!(
            w.light
                .as_ref()
                .expect("Let there be darkness!")
                .get_intensity(),
            light.get_intensity()
        );
        assert_eq!(
            w.light
                .as_ref()
                .expect("Let there be darkness!")
                .get_position(),
            light.get_position()
        );
        assert_eq!(w.objects[0].get_material(), s1.get_material());
        assert_eq!(w.objects[0].get_transform(), s1.get_transform());
        assert_eq!(w.objects[1].get_material(), s2.get_material());
        assert_eq!(w.objects[1].get_transform(), s2.get_transform());
    }

    #[test]
    fn intersect_world_ray_world() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect_world(&r);
        assert!(xs.is_some());
        let xs = xs.unwrap();

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }
}
