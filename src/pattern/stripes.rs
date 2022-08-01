use crate::{Pattern, Point, Shape, Transformation, BLACK, RGB, WHITE};

/// This generates stripes for any Shape.
#[derive(Debug, Clone, Copy)]
pub struct Stripes {
    /// Color 1.
    pub a: RGB,

    /// Color 2.
    pub b: RGB,

    ///
    pub transform: Transformation,
}

impl Stripes {
    /// Generate a new (default) Stripe Pattern.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate a Stripe Pattern with given RGBs.
    pub fn stripe_pattern(a: RGB, b: RGB) -> Self {
        Self {
            a,
            b,
            transform: Transformation::new(),
        }
    }

    /// Give back the RGB value of the Stripe at point.
    pub fn stripe_at(&self, point: Point) -> RGB {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    /// Return the RGB value for the given Shape and Point.
    pub fn stripe_at_object(&self, object: &dyn Shape, world_point: Point) -> RGB {
        let object_point = object
            .get_transform()
            .init()
            .inverse(4)
            .expect("Object transform should be invertible")
            * world_point;
        let pattern_point = self
            .transform
            .init()
            .inverse(4)
            .expect("Pattern transform should be invertible")
            * object_point;

        self.stripe_at(pattern_point)
    }

    /// Set the transformation matrix.
    pub fn set_pattern_transform(&mut self, t: Transformation) {
        self.transform = t;
    }
}

impl Default for Stripes {
    fn default() -> Self {
        Self {
            a: WHITE,
            b: BLACK,
            transform: Transformation::default(),
        }
    }
}

impl PartialEq for Stripes {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Pattern for Stripes {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Sphere, Transformation};

    #[test]
    fn create_stripe() {
        let pattern = Stripes::stripe_pattern(WHITE, BLACK);

        assert_eq!(pattern.a, WHITE);
        assert_eq!(pattern.b, BLACK);
    }

    #[test]
    fn default_stripe() {
        let pattern = Stripes::new();

        assert_eq!(pattern.a, WHITE);
        assert_eq!(pattern.b, BLACK);
    }

    #[test]
    fn const_y_stripe() {
        let pattern = Stripes::new();

        assert_eq!(pattern.stripe_at(Point::new(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 2.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 3.0, 0.0)), WHITE);
    }

    #[test]
    fn const_z_stripe() {
        let pattern = Stripes::new();

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 2.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 3.0)), WHITE);
    }

    #[test]
    fn alt_x_stripe() {
        let pattern = Stripes::new();

        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.stripe_at(Point::new(-1.1, 0.0, 0.0)), WHITE);
    }

    #[test]
    fn object_transform_stripe() {
        let mut object = Sphere::new();
        object.set_transform(Transformation::new().scaling(2.0, 2.0, 2.0));
        let pattern = Stripes::new();
        let c = pattern.stripe_at_object(&object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, WHITE);
    }

    #[test]
    fn pattern_transform_stripe() {
        let object = Sphere::new();
        let mut pattern = Stripes::new();
        pattern.set_pattern_transform(Transformation::new().scaling(2.0, 2.0, 2.0));
        let c = pattern.stripe_at_object(&object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, WHITE);
    }

    #[test]
    fn pattern_object_transform_stripe() {
        let mut object = Sphere::new();
        object.set_transform(Transformation::new().scaling(2.0, 2.0, 2.0));
        let mut pattern = Stripes::new();
        pattern.set_pattern_transform(Transformation::new().scaling(2.0, 2.0, 2.0));
        let c = pattern.stripe_at_object(&object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, WHITE);
    }
}
