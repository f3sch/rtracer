use crate::*;
use uuid::Uuid;

/// Ring pattern.
#[derive(Debug)]
pub struct Ring {
    /// Id.
    uuid: Uuid,

    /// Color 1.
    a: RGB,

    /// Color 2.
    b: RGB,

    /// Transformation matrix.
    transform: Transformation,
}

impl Ring {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            a: WHITE,
            b: BLACK,
            transform: Transformation::new(),
        }
    }

    pub fn ring_pattern(a: RGB, b: RGB) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            a,
            b,
            transform: Transformation::new(),
        }
    }
}

impl Pattern for Ring {
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
        let tmp = (point.x * point.x + point.y * point.y).sqrt().floor();
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
    fn ring_extend_pattern() {
        let pattern = Ring::ring_pattern(WHITE, BLACK);

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.708, 0.0, 0.708)), WHITE);
    }
}
