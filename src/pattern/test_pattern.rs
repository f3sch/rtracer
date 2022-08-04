use crate::*;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct TestPattern {
    transform: Transformation,
}

impl TestPattern {
    pub fn new() -> Self {
        Self {
            transform: Transformation::new(),
        }
    }
}

impl Pattern for TestPattern {
    fn id(&self) -> uuid::Uuid {
        Uuid::nil()
    }

    fn get_transform(&self) -> Transformation {
        self.transform
    }

    fn set_transform(&mut self, t: Transformation) {
        self.transform = t;
    }

    fn pattern_at(&self, point: Point) -> RGB {
        RGB {
            red: point.x,
            green: point.y,
            blue: point.z,
        }
    }
}
