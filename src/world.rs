use crate::*;

/// A world holds every shape and a light source.
pub struct World {
    /// All Shapes contain in a World.
    objects: Vec<Box<dyn Shape>>,

    /// The light source.
    light: Option<PointLight>,
}

impl World {
    /// In the beginning I created nothing.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            light: None,
        }
    }

    /// Set the light source of the world.
    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }

    /// Add objects/shapes to a world.
    pub fn add_object(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }

    /// Return a reference to an object inside the world identified by the index.
    pub fn get_object(&self, index: usize) -> Option<&dyn Shape> {
        match self.objects.get(index) {
            Some(obj) => Some(obj.as_ref()),
            None => None,
        }
    }

    /// Return a mut reference to an object inside the world identified by the index.
    pub fn get_object_mut(&mut self, index: usize) -> Option<&mut dyn Shape> {
        match self.objects.get_mut(index) {
            Some(obj) => Some(obj.as_mut()),
            None => None,
        }
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

    /// Compute the color at the intersection.
    pub fn shade_hit(&self, comps: &Computation, remaining: usize) -> RGB {
        let shadowed = self.is_shadowed(comps.over_point);
        let surface = comps.object.get_material().lightning(
            comps.object,
            self.light.expect("World has no light!"),
            comps.point,
            comps.eyev,
            comps.normalv,
            shadowed,
        );
        let reflected = self.reflected_color(&comps, remaining);

        surface + reflected
    }

    /// Compute the Color of a Ray.
    pub fn color_at(&self, ray: &Ray, remaining: usize) -> RGB {
        match self.intersect_world(ray) {
            Some(xs) => match hit(xs) {
                Some(i) => {
                    let comps = i.prepare_computations(&ray);
                    self.shade_hit(&comps, remaining)
                }
                None => BLACK,
            },
            None => BLACK,
        }
    }

    /// Test if a point is in shadows.
    pub fn is_shadowed(&self, p: Point) -> bool {
        let v = self.light.expect("World has no light!").get_position() - p;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(p, direction);
        if let Some(intersections) = self.intersect_world(&r) {
            if let Some(h) = hit(intersections) {
                if h.t < distance {
                    return true;
                }
            }
        }

        false
    }

    /// Compute the reflected color.
    pub fn reflected_color(&self, comps: &Computation, remaining: usize) -> RGB {
        if comps.object.get_material().reflective == 0.0 || remaining == 0 {
            return BLACK;
        }

        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray, remaining - 1);

        color * comps.object.get_material().reflective
    }
}

impl Default for World {
    fn default() -> Self {
        let mut w = World::new();

        w.light = Some(PointLight::new(Point::new(-10.0, 10.0, -10.0), WHITE));
        let mut s1 = Sphere::new();
        let mut m1 = Material::default();
        m1.color = RGB::new(0.8, 1.0, 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;
        s1.set_material(m1);
        add_object!(w, s1);

        let mut s2 = Sphere::new();
        let t2 = Transformation::new().scaling(0.5, 0.5, 0.5);
        s2.set_transform(t2);
        add_object!(w, s2);

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
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), WHITE);
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

    #[test]
    fn shading_outside_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w
            .get_object(0)
            .expect("Default world should have two shapes!");
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps, 0);

        assert_eq!(c, RGB::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_inside_intersection() {
        let mut w = World::default();
        w.light = Some(PointLight::new(Point::new(0.0, 0.25, 0.0), WHITE));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w
            .get_object(1)
            .expect("Default world should have two shapes!");
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps, 0);

        assert_eq!(c, RGB::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_miss_world() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r, 0);

        assert_eq!(c, BLACK);
    }

    #[test]
    fn color_hit_world() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r, 0);

        assert_eq!(c, RGB::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_behind_intersection_world() {
        let mut w = World::default();
        {
            let outer = w
                .get_object_mut(0)
                .expect("First object must exists in default world!");
            outer.get_material_mut().ambient = 1.0;
            let inner = w
                .get_object_mut(1)
                .expect("First object must exists in default world!");
            inner.get_material_mut().ambient = 1.0;
        }
        let inner = w
            .get_object(1)
            .expect("First object must exists in default world!");
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(&r, 0);

        assert_eq!(c, inner.get_material().color);
    }

    #[test]
    fn point_collinear_light_world() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn point_object_light_world() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(p));
    }

    #[test]
    fn point_light_object_world() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn object_point_light_world() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_shadow_world() {
        let mut w = World::new();
        w.light = Some(PointLight::new(Point::new(0.0, 0.0, -10.0), WHITE));
        let s1 = Sphere::new();
        add_object!(w, s1);
        let mut s2 = Sphere::new();
        s2.set_transform(Transformation::new().translation(0.0, 0.0, -10.0));
        add_object!(w, s2);
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, w.get_object(1).expect("Where is it?"));
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps, 0);

        assert_eq!(c, RGB::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn nonreflective_object() {
        let mut w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        {
            let shape = w.get_object_mut(1).expect("Default world has 2 spheres");
            shape.get_material_mut().ambient = 1.0;
        }
        let i = Intersection::new(1.0, w.get_object(1).expect("Default world has 2 spheres"));
        let comps = i.prepare_computations(&r);
        let color = w.reflected_color(&comps, 0);

        assert_eq!(color, BLACK);
    }

    #[test]
    fn reflective_object() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.get_material_mut().reflective = 0.5;
        shape.set_transform(Transformation::new().translation(0.0, -1.0, 0.0));
        add_object!(w, shape);
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2_f64.sqrt() / 2.0), 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(
            2_f64.sqrt(),
            w.get_object(2).expect("I just added this plane?"),
        );
        let comps = i.prepare_computations(&r);
        let color = w.reflected_color(&comps, 4);

        assert_eq!(color, RGB::new(0.19032, 0.2379, 0.14274));
    }

    #[test]
    fn shade_hit_reflective_object() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.get_material_mut().reflective = 0.5;
        shape.set_transform(Transformation::new().translation(0.0, -1.0, 0.0));
        add_object!(w, shape);
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2_f64.sqrt() / 2.0), 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(
            2_f64.sqrt(),
            w.get_object(2).expect("I just added this plane?"),
        );
        let comps = i.prepare_computations(&r);
        let color = w.shade_hit(&comps, 4);

        assert_eq!(color, RGB::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn infinite_reflection_world() {
        let mut w = World::new();
        w.set_light(PointLight::new(Point::new(0.0, 0.0, 0.0), WHITE));
        let mut lower = Plane::new();
        lower.get_material_mut().reflective = 1.0;
        lower.set_transform(Transformation::new().translation(0.0, -1.0, 0.0));
        add_object!(w, lower);
        let mut upper = Plane::new();
        upper.get_material_mut().reflective = 1.0;
        upper.set_transform(Transformation::new().translation(0.0, 1.0, 0.0));
        add_object!(w, upper);
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));

        w.color_at(&r, 4);
    }

    #[test]
    fn reflective_limit_object() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.get_material_mut().reflective = 0.5;
        shape.set_transform(Transformation::new().translation(0.0, -1.0, 0.0));
        add_object!(w, shape);
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -(2_f64.sqrt() / 2.0), 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(
            2_f64.sqrt(),
            w.get_object(2).expect("I just added this plane?"),
        );
        let comps = i.prepare_computations(&r);
        let color = w.reflected_color(&comps, 0);

        assert_eq!(color, BLACK);
    }
}
