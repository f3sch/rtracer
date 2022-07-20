use crate::color::RGB;

/// Canvas object
#[derive(Debug)]
pub struct Canvas {
    /// Width of the Canvas.
    width: usize,
    /// Height of the Canvas.
    height: usize,
    /// Pixels of the Canvas. TODO Avoid heap allocations!
    pixels: Vec<Vec<RGB>>,
}

impl Canvas {
    /// Create a Canvas.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![RGB::new(0.0, 0.0, 0.0); width]; height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: RGB) {
        self.pixels[y][x] = color;
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        let mut counter: usize = 0;

        // Header
        ppm.push_str("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");

        // Colors
        for y in 0..self.height {
            for x in 0..self.width {
                let tmp = &self.pixels[y][x].ppm_clamp();
                counter += tmp.len();
                ppm.push_str(tmp);
                if counter >= 70 {
                    ppm.push_str("\n");
                    counter = 0;
                } else {
                    ppm.push_str(" ");
                }
            }
            ppm.push_str("\n"); // end with newline
            counter = 0;
        }

        ppm
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for y in 0..c.height {
            for x in 0..c.width {
                assert_eq!(c.pixels[y][x], RGB::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn write_pixel_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = RGB::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);

        assert_eq!(c.pixels[3][2], red);
    }

    #[test]
    #[should_panic]
    fn write_pixel_fail_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = RGB::new(1.0, 0.0, 0.0);
        c.write_pixel(19, 1, red);
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
        let correct = String::from("P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 127 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n");

        assert_eq!(ppm, correct);
    }

    #[test]
    fn ppm_line_limit_canvas(){
        let mut c = Canvas::new(10, 2);

        for y in 0..c.height {
            for x in 0..c.width {
                c.write_pixel(x, y, RGB::new(1.0, 0.8, 0.6));
            }
        }
        let ppm = c.to_ppm();
        let correct = String::from("P3\n10 2\n255\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 \n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 \n");

        assert_eq!(ppm, correct);
    }
}
