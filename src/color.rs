use float_eq::float_eq;
use std::ops::{Add, Mul, Sub};

/// RGB color object
#[derive(Debug, Clone, Copy)]
pub struct RGB {
    /// Red color grade [0,1]
    pub red: f64,
    /// Green color grade [0,1]
    pub green: f64,
    /// Blue color grade [0,1]
    pub blue: f64,
}

impl RGB {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}

impl Add for RGB {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for RGB {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<f64> for RGB {
    type Output = Self;
    fn mul(self, s: f64) -> Self::Output {
        Self {
            red: self.red * s,
            green: self.green * s,
            blue: self.blue * s,
        }
    }
}

impl Mul<RGB> for RGB {
    type Output = Self;
    fn mul(self, rhs: RGB) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl PartialEq for RGB {
    fn eq(&self, other: &Self) -> bool {
        float_eq!(self.red, other.red, abs <= 0.00001)
            && float_eq!(self.green, other.green, abs <= 0.00001)
            && float_eq!(self.blue, other.blue, abs <= 0.00001)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_color() {
        let c = RGB::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn add_color() {
        let c1 = RGB::new(0.9, 0.6, 0.75);
        let c2 = RGB::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, RGB::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn sub_color() {
        let c1 = RGB::new(0.9, 0.6, 0.75);
        let c2 = RGB::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, RGB::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn mul_color_color() {
        let c1 = RGB::new(1.0,0.2,0.4);
        let c2 = RGB::new(0.9,1.0,0.1);

        assert_eq!(c1 * c2, RGB::new(0.9,0.2,0.04));
    }

    #[test]
    fn mul_scalar_color() {
        let c = RGB::new(0.2, 0.3, 0.4);
        let s = 2.0;

        assert_eq!(c * s, RGB::new(0.4, 0.6, 0.8));
    }
}
