use crate::{float_cmp, float_eq, shapes::*, Computation, Ray};

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

    /// Pre-compute some information.
    pub fn prepare_computations(&self, r: &Ray) -> Computation {
        let point = r.position(self.t);
        let eyev = -r.direction();
        let mut normalv = self.object.normal_at(point);
        let mut inside = false;

        if normalv.dot(eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        Computation {
            t: self.t,
            object: self.object,
            point,
            eyev,
            normalv,
            inside,
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

pub fn hit(mut xs: Vec<Intersection>) -> Option<Intersection> {
    xs.sort();
    match xs.iter().filter(|x| x.t.is_sign_positive()).min() {
        None => None,
        Some(h) => Some(h.clone()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Point, Sphere, Vector};

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
        let i = hit(xs).unwrap();

        assert_eq!(i, i1);
    }

    #[test]
    fn negative_hit() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i2, i1];
        let i = hit(xs).unwrap();

        assert_eq!(i, i2);
    }

    #[test]
    fn negative_all_hit() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2, i1];

        assert!(hit(xs).is_none());
    }

    #[test]
    fn lowest_nonnegative_hit() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];
        let i = hit(xs).unwrap();

        assert_eq!(i, i4);
    }

    #[test]
    fn precompute_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let i = Intersection::new(4.0, &s);
        let comps = i.prepare_computations(&r);

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
        let comps = i.prepare_computations(&r);

        assert!(!comps.inside);
    }

    #[test]
    fn precompute_inside_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(&r);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }
}
