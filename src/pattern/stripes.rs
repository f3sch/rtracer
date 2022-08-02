use crate::{float_eq, Pattern, Point, Transformation, BLACK, RGB, WHITE};
use uuid::Uuid;

/// This generates stripes for any Shape.
#[derive(Debug, Clone, Copy)]
pub struct Stripes {
    /// Unique identifier for pattern.
    pub uuid: Uuid,

    /// Color 1.
    pub a: RGB,

    /// Color 2.
    pub b: RGB,

    /// Transformation matrix.
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
            uuid: Uuid::new_v4(),
            a,
            b,
            transform: Transformation::new(),
        }
    }

    /// Give back the RGB value of the Stripe at point.
    pub fn stripe_at(&self, point: Point) -> RGB {
        if float_eq(point.x.floor() % 2.0, 0.0) {
            self.a
        } else {
            self.b
        }
    }
}

impl Default for Stripes {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            a: WHITE,
            b: BLACK,
            transform: Transformation::default(),
        }
    }
}

impl PartialEq for Stripes {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Pattern for Stripes {
    fn id(&self) -> Uuid {
        self.uuid
    }

    fn get_transform(&self) -> Transformation {
        self.transform
    }

    fn set_transform(&mut self, t: Transformation) {
        self.transform = t;
    }

    fn pattern_at(&self, point: Point) -> RGB {
        self.stripe_at(point)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}
