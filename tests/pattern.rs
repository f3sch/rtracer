use rtracer::*;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
struct TestPattern {
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

#[test]
fn default_transformation_pattern() {
    let pattern = TestPattern::new();

    assert_eq!(pattern.transform.init(), IDENTITY);
}

#[test]
fn set_transform_pattern() {
    let mut pattern = TestPattern::new();
    pattern.set_transform(Transformation::new().translation(1.0, 2.0, 3.0));

    assert_eq!(
        pattern.get_transform(),
        Transformation::new().translation(1.0, 2.0, 3.0)
    );
}

#[test]
fn object_transform_pattern() {
    let mut shape = Sphere::new();
    shape.set_transform(Transformation::new().scaling(2.0, 2.0, 2.0));
    let pattern = TestPattern::new();
    let c = pattern.pattern_at_shape(&shape, Point::new(2.0, 3.0, 4.0));

    assert_eq!(c, RGB::new(1.0, 1.5, 2.0));
}

#[test]
fn pattern_transform_pattern() {
    let shape = Sphere::new();
    let mut pattern = TestPattern::new();
    pattern.set_transform(Transformation::new().scaling(2.0, 2.0, 2.0));
    let c = pattern.pattern_at_shape(&shape, Point::new(2.0, 3.0, 4.0));

    assert_eq!(c, RGB::new(1.0, 1.5, 2.0));
}

#[test]
fn pattern_object_transform_pattern() {
    let mut shape = Sphere::new();
    shape.set_transform(Transformation::new().scaling(2.0, 2.0, 2.0));
    let mut pattern = TestPattern::new();
    pattern.set_transform(Transformation::new().translation(0.5, 1.0, 1.5));
    let c = pattern.pattern_at_shape(&shape, Point::new(2.5, 3.0, 3.5));

    assert_eq!(c, RGB::new(0.75, 0.5, 0.25));
}
