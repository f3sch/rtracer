use crate::float_eq;
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

    pub fn ppm_clamp(&self) -> String {
        let c_red = clamp(self.red);
        let c_green = clamp(self.green);
        let c_blue = clamp(self.blue);
        format!("{} {} {}", c_red, c_green, c_blue)
    }

    pub fn from_u8(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red: red as f64 / 255.0,
            green: green as f64 / 255.0,
            blue: blue as f64 / 255.0,
        }
    }

    /// Get an array of the  parts of a `Color` as [`u8`] in string format. The
    pub fn rgb_string_array(&self) -> [String; 3] {
        [
            format!("{}", clamp(self.red)),
            format!("{}", clamp(self.green)),
            format!("{}", clamp(self.blue)),
        ]
    }
}

// clamp function for RGB
fn clamp(c: f64) -> u8 {
    let c = c * 255.0;
    if c > 255.0 {
        255u8
    } else if c < 0.0 {
        0u8
    } else {
        c as u8
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
        float_eq(self.red, other.red)
            && float_eq(self.green, other.green)
            && float_eq(self.blue, other.blue)
    }
}

pub const BLACK: RGB = RGB {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

pub const WHITE: RGB = RGB {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

pub const RED: RGB = RGB {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
};

pub const GREEN: RGB = RGB {
    red: 0.0,
    green: 1.0,
    blue: 0.0,
};

pub const BLUE: RGB = RGB {
    red: 0.0,
    green: 0.0,
    blue: 1.0,
};

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
        let c1 = RGB::new(1.0, 0.2, 0.4);
        let c2 = RGB::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, RGB::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn mul_scalar_color() {
        let c = RGB::new(0.2, 0.3, 0.4);
        let s = 2.0;

        assert_eq!(c * s, RGB::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn clamp_color() {
        let a = 1.5;
        let b = 0.7;
        let c = -0.2;

        assert_eq!(clamp(a), 255);
        assert_eq!(clamp(b), 178);
        assert_eq!(clamp(c), 0);
    }

    #[test]
    fn clamp_rgb_color() {
        let c = RGB::new(-0.2, 0.7, 1.5);

        assert_eq!(String::from("0 178 255"), c.ppm_clamp());
    }
}
