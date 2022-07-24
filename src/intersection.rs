use std::fmt::Debug;
use crate::shapes::*;

/// Generic intersection object, which works on all shapes that
/// implement the 'Shape' trait.
/// An Intersection existence makes only sense when the underlying
/// object exists.
/// So the lifetime is tied to the intersected object.
pub struct Intersection<'a, T>
where
    T: Shape<T> + PartialEq + Debug,
{
    /// The t value of the intersection.
    pub t: f64,
    /// A reference to the object that was intersected.
    pub object: &'a T,
}

impl<'a, T> Intersection<'a, T>
where
    T: Shape<T> + PartialEq + Debug,
{
    /// Create a new Intersection with a reference to the object.
    pub fn new(t: f64, object: &'a T) -> Self {
        Self { t, object }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Sphere;

    #[test]
    fn sphere_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn intersections(){
        let s = Sphere::new();
        let i1 = Intersection::new(1.0,&s);
        let i2 = Intersection::new(2.0,&s);
        let xs = vec![i1,i2];

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }
}
