use crate::{Point, RGB};
use std::fmt::Debug;

/// A PointLight is light with no size, exisiting at a single
/// point in space.
/// It is also defined by its intensity.
#[derive(Debug,Clone, Copy)]
pub struct PointLight {
    /// Essentially the same as brightness.
    intensity: RGB,

    /// The singular position of the light source.
    position: Point,
}

impl PointLight {
    /// Create a new PointLight.
    pub fn new(position: Point, intensity: RGB) -> Self {
        Self {
            intensity,
            position,
        }
    }

    pub fn get_intensity(&self) -> RGB {
        self.intensity
    }

    pub fn get_position(self) -> Point {
        self.position
    }
}

impl PartialEq for PointLight {
    fn eq(&self, other: &Self) -> bool {
        self.intensity == other.intensity && self.position == other.position
    }
}

#[cfg(test)]
mod test {
    use crate::WHITE;

    use super::*;

    #[test]
    fn create_point_light() {
        let intensity = WHITE;
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
