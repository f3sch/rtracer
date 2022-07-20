use crate::color::RGB;

/// Canvas object
pub struct Canvas {
    /// Height of the Canvas.
    height: usize,
    /// Width of the Canvas.
    width: usize,
    /// Pixels of the Canvas. TODO Avoid heap allocations!
    pixels: Vec<Vec<RGB>>,
}

impl Canvas {
    /// Create a Canvas.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            height,
            width,
            pixels: vec![vec![RGB::new(0.0, 0.0, 0.0); width]; height],
        }
    }

    pub fn write_pixel(&mut self, x:usize  , y:usize, color: RGB) {
        self.pixels[x][y] = color;
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
        for x in 0..c.height {
            for y in 0..c.width {
                assert_eq!(c.pixels[x][y], RGB::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn write_pixel_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = RGB::new(1.0,0.0,0.0);
        c.write_pixel(2, 3, red);

        assert_eq!(c.pixels[2][3], red);
    }
}
