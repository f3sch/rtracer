use crate::{shapes::Shape, Point, Vector};

/// A Computation encapsulates some pre-compute information of an intersection and an object.
pub struct Computation<'a> {
    /// Distance from the origin of a Ray to the intersection.
    pub t: f64,

    /// The object intersected by a Ray.
    pub object: &'a dyn Shape,

    /// Point in world space where the Intersection occurred.
    pub point: Point,

    /// Eye Vector of the surface of the object.
    pub eyev: Vector,

    /// Normal Vector of the surface of the object.
    pub normalv: Vector,

    /// Is the intersection inside of the shape?
    pub inside: bool,

    /// over_point represents what?
    pub over_point: Point,

    /// under_point represents what?
    pub under_point: Point,

    /// where to reflect
    pub reflectv: Vector,

    /// Refraction 1.
    pub n1: f64,

    /// Refraction 2.
    pub n2: f64,
}

impl Computation<'_> {
    /// Fresnel effect.
    pub fn schlick(&self) -> f64 {
        // find the cosine of the angle between the eye and normal vector
        let mut cos = self.eyev.dot(self.normalv);

        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            // computer cosine of theta_t using trig identity
            // when n1 > n2 use cos(theta_t) instead
            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}
