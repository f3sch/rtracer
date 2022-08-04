use crate::*;
use uuid::Uuid;

/// Checkers pattern.
#[derive(Debug, Clone, Copy)]
pub struct Checkers {
    /// Id.
    uuid: Uuid,

    /// Color 1.
    a: RGB,

    /// Color 2.
    b: RGB,

    /// Transformation matrix.
    transform: Transformation,
}

impl Checkers {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            a: WHITE,
            b: BLACK,
            transform: Transformation::new(),
        }
    }

    pub fn checkers_pattern(a: RGB, b: RGB) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            a,
            b,
            transform: Transformation::new(),
        }
    }
}

impl Pattern for Checkers {
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
        let tmp = point.x.floor() + point.y.floor() + point.z.floor();
        if float_eq(tmp % 2.0, 0.0) {
            return self.a;
        }

        self.b
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn checker_x_pattern() {
        let pattern = Checkers::checkers_pattern(WHITE, BLACK);

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new(1.01, 0.0, 0.0)), BLACK);
    }

    #[test]
    fn checker_y_pattern() {
        let pattern = Checkers::checkers_pattern(WHITE, BLACK);

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 1.01, 0.0)), BLACK);
    }
    #[test]
    fn checker_z_pattern() {
        let pattern = Checkers::checkers_pattern(WHITE, BLACK);

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 1.01)), BLACK);
    }
}
