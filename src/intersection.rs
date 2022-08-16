use crate::*;

/// Generic intersection object, which works on all shapes that
/// implement the 'Shape' trait.
/// An Intersection existence makes only sense when the underlying
/// object exists.
/// So the lifetime is tied to the intersected object.
#[derive(Clone, Copy, Debug)]
pub struct Intersection<'a> {
    /// The t value of the intersection.
    pub t: f64,
    /// A reference to the object that was intersected.
    pub object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    /// Create a new Intersection with a reference to the object.
    pub fn new(t: f64, object: &'a dyn Shape) -> Self {
        Self { t, object }
    }

    pub fn hit(xs: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
        xs.iter().filter(|x| x.t >= 0.0).min()
    }

    /// Pre-compute some information.
    pub fn prepare_computations(
        &self,
        r: &Ray,
        xs: &Vec<Intersection>,
        w: Option<&World>,
    ) -> Computation {
        let point = r.position(self.t);
        let eyev = -r.direction();
        let mut normalv = self.object.normal_at(point, w);
        let mut inside = false;

        if normalv.dot(eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        let over_point = point + normalv * EPSILON;
        let under_point = point - normalv * EPSILON;
        let reflectv = r.direction().reflect(normalv);

        let mut n1 = 0.0;
        let mut n2 = 0.0;
        let mut container: Vec<&dyn Shape> = Vec::new();
        for i in xs {
            if i == self {
                if container.is_empty() {
                    n1 = 1.0;
                } else if let Some(object) = container.last() {
                    n1 = object.get_material().refractive_index;
                }
            }

            if container.contains(&i.object) {
                container = container.into_iter().filter(|o| *o != i.object).collect();
            } else {
                container.push(i.object);
            }

            if i == self {
                if container.is_empty() {
                    n2 = 1.0;
                } else if let Some(object) = container.last() {
                    n2 = object.get_material().refractive_index;
                }

                break;
            }
        }

        Computation {
            t: self.t,
            object: self.object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
            under_point,
            reflectv,
            n1,
            n2,
        }
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.t, other.t) && self.object.eq(other.object)
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(float_cmp(self.t, other.t))
    }
}

impl Eq for Intersection<'_> {}

impl Ord for Intersection<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        float_cmp(self.t, other.t)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Point, Sphere, Transformation, Vector, EPSILON};

    #[test]
    fn sphere_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert!(i.object.eq(&s));
    }

    #[test]
    fn intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn positive_hit() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];
        let i = *Intersection::hit(&xs).unwrap();

        assert_eq!(i, i1);
    }

    #[test]
    fn negative_hit() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i2, i1];
        let i = *Intersection::hit(&xs).unwrap();

        assert_eq!(i, i2);
    }

    #[test]
    fn negative_all_hit() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2, i1];

        assert!(Intersection::hit(&xs).is_none());
    }

    #[test]
    fn lowest_nonnegative_hit() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];
        let i = *Intersection::hit(&xs).unwrap();

        assert_eq!(i, i4);
    }

    #[test]
    fn precompute_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let i = Intersection::new(4.0, &s);
        let xs = &vec![i];
        let comps = i.prepare_computations(&r, xs, None);

        assert_eq!(comps.t, i.t);
        assert!(comps.object.eq(&s));
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn precompute_outside_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let xs = &vec![i];
        let comps = i.prepare_computations(&r, xs, None);

        assert!(!comps.inside);
    }

    #[test]
    fn precompute_inside_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);
        let xs = &vec![i];
        let comps = i.prepare_computations(&r, xs, None);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_offset_point_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Sphere::new();
        shape.set_transform(Transformation::new().translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);
        let xs = &vec![i];
        let comps = i.prepare_computations(&r, xs, None);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precompute_reflect_intersection() {
        let shape = Plane::new();
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -(2_f64.sqrt() / 2.0), 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), &shape);
        let xs = &vec![i];
        let comps = i.prepare_computations(&r, xs, None);

        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn find_n1_n2_intersection() {
        let mut a = Sphere::glass_sphere();
        a.set_transform(Transformation::new().scaling(2.0, 2.0, 2.0));
        a.get_material_mut().refractive_index = 1.5;
        let ia1 = Intersection::new(2.0, &a);
        let ia2 = Intersection::new(6.0, &a);
        let mut b = Sphere::glass_sphere();
        b.set_transform(Transformation::new().translation(0.0, 0.0, -0.25));
        b.get_material_mut().refractive_index = 2.0;
        let ib1 = Intersection::new(2.75, &b);
        let ib2 = Intersection::new(4.75, &b);
        let mut c = Sphere::glass_sphere();
        c.set_transform(Transformation::new().translation(0.0, 0.0, 0.25));
        c.get_material_mut().refractive_index = 2.5;
        let ic1 = Intersection::new(3.25, &c);
        let ic2 = Intersection::new(5.25, &c);
        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
        let xs = vec![ia1, ib1, ic1, ib2, ic2, ia2];
        let expected = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for i in 0..5 {
            let comps = xs[i].prepare_computations(&r, &xs, None);
            assert_eq!(expected[i].0, comps.n1);
            assert_eq!(expected[i].1, comps.n2);
        }
    }

    #[test]
    fn under_point_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Sphere::glass_sphere();
        shape.set_transform(Transformation::new().translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);
        let xs = &vec![i];
        let comps = i.prepare_computations(&r, xs, None);

        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick_total_internal_reflection_intersection() {
        let shape = Sphere::glass_sphere();
        let r = Ray::new(
            Point::new(0.0, 0.0, 2_f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let xs = vec![
            Intersection::new(-2_f64.sqrt() / 2.0, &shape),
            Intersection::new(2_f64.sqrt() / 2.0, &shape),
        ];
        let comps = xs[1].prepare_computations(&r, &xs, None);
        let reflectance = comps.schlick();

        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn schlick_perpendicular_intersection() {
        let shape = Sphere::glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-1.0, &shape),
            Intersection::new(1.0, &shape),
        ];
        let comps = xs[1].prepare_computations(&r, &xs, None);
        let reflectance = comps.schlick();

        assert!(float_eq(reflectance, 0.04));
    }

    #[test]
    fn schlick_n1_smaller_n1_intersection() {
        let shape = Sphere::glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs = vec![Intersection::new(1.8589, &shape)];
        let comps = xs[0].prepare_computations(&r, &xs, None);
        let reflectance = comps.schlick();

        assert!(float_eq(reflectance, 0.48873));
    }
}
