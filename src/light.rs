use crate::{Point, RGB};

/// General light trait.
pub trait Light {
    /// Get the intensity of the light.
    fn get_intensity(&self) -> RGB;

    /// Get the position of the light.
    fn get_position(&self) -> Point;
}

/// A PointLight is light with no size, exisiting at a single
/// point in space.
/// It is also defined by its intensity.
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
}

impl Light for PointLight {
    fn get_intensity(&self) -> RGB {
        self.intensity
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_point_light() {
        let intensity = RGB::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
