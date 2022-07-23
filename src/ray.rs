use crate::{Point, Vector};

/// Ray implementation.
/// Each ray created by your ray tracer will have a starting point
/// called the origin, and a vector called the direction which says
/// where it points.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    /// Starting point.
    origin: Point,
    /// Direction from origin.
    direction: Vector,
}

impl Ray {
    /// Create a new Ray.
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    /// This function should compute the point at the given distance
    /// 't' along the ray.
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn position_ray() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }
}
