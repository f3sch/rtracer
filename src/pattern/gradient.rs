use crate::*;
use uuid::Uuid;

/// Gradient pattern.
#[derive(Debug, Clone, Copy)]
pub struct Gradient {
    /// Id.
    uuid: Uuid,

    /// Color 1.
    a: RGB,

    /// Color 2.
    b: RGB,

    /// Transformation matrix.
    transform: Transformation,
}

impl Gradient {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            a: WHITE,
            b: BLACK,
            transform: Transformation::new(),
        }
    }

    pub fn gradient_pattern(a: RGB, b: RGB) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            a,
            b,
            transform: Transformation::new(),
        }
    }
}

impl Pattern for Gradient {
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
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();

        self.a + distance * fraction
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gradient_linear_pattern() {
        let pattern = Gradient::gradient_pattern(WHITE, BLACK);

        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(
            pattern.pattern_at(Point::new(0.25, 0.0, 0.0)),
            RGB::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.5, 0.0, 0.0)),
            RGB::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(Point::new(0.75, 0.0, 0.0)),
            RGB::new(0.25, 0.25, 0.25)
        );
    }
}
