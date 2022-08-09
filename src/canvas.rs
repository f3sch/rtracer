use crate::{color::RGB, BLACK};

const MAXIMUM_PPM_LINE_LENGTH: usize = 70;

/// Canvas object
#[derive(Debug)]
pub struct Canvas {
    /// Width of the Canvas.
    pub width: usize,
    /// Height of the Canvas.
    pub height: usize,
    /// Pixels of the Canvas. TODO Avoid heap allocations!
    pub pixels: Vec<RGB>,
}

impl Canvas {
    /// Create a Canvas.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![BLACK; height * width],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: RGB) {
        let i = x + y * self.width;
        self.pixels[i] = color;
    }

    pub fn to_ppm(&self) -> String {
        let mut buffer = ["P3", &format!("{} {}", self.width, self.height), "255"].join("\n");
        buffer.push('\n');

        let mut col_counter = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixel_at(x, y);

                for c in pixel.rgb_string_array().iter() {
                    if col_counter + c.len() + 1 > MAXIMUM_PPM_LINE_LENGTH {
                        buffer += "\n";
                        col_counter = 0;
                    }
                    if col_counter > 0 {
                        buffer += " ";
                    }
                    buffer += c;
                    col_counter += c.len() + 1;
                }
            }
            buffer.push('\n');
            col_counter = 0;
        }
        buffer.push('\n');

        buffer
    }

    /// Return the color at the given pixel.
    pub fn pixel_at(&self, x: usize, y: usize) -> RGB {
        let i = x + y * self.width;

        self.pixels[i]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{BLACK, RED};

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for y in 0..c.height {
            for x in 0..c.width {
                assert_eq!(c.pixels[x + y * c.width], BLACK);
            }
        }
    }

    #[test]
    fn write_pixel_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = RED;
        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    #[should_panic]
    fn write_pixel_fail_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = RED;
        c.write_pixel(109, 10, red);
    }

    #[test]
    fn ppm_construct_canvas() {
        let mut c = Canvas::new(5, 3);
        let c1 = RGB::new(1.5, 0.0, 0.0);
        let c2 = RGB::new(0.0, 0.5, 0.0);
        let c3 = RGB::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        let correct = String::from("P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 127 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n\n");

        assert_eq!(ppm, correct);
    }

    #[test]
    fn ppm_line_limit_canvas() {
        let mut c = Canvas::new(10, 2);

        for y in 0..c.height {
            for x in 0..c.width {
                c.write_pixel(x, y, RGB::new(1.0, 0.8, 0.6));
            }
        }
        let ppm = c.to_ppm();
        let correct = String::from("P3\n10 2\n255\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n\n");

        assert_eq!(ppm, correct);
    }
}
