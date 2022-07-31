use crate::{Canvas, Point, Ray, Transformation, World};
use progress_bar::*;

/// Just like a real camera, the virtual camera allows moving around in the scene.
pub struct Camera {
    /// hsize is the horizontal size (in pixels) of the canvas that the picture will be rendered to.
    pub hsize: usize,

    /// vsize is the canvas’s vertical size (in pixels).
    pub vsize: usize,

    /// field_of_view is an angle that describes how much the camera can see.
    /// When the field of view is small, the view will be “zoomed in,”
    /// magnifying a smaller area of the scene.
    pub field_of_view: f64,

    /// transform is a matrix describing how the world should be oriented
    /// relative to the camera. This is usually a view transformation like you
    /// implemented in the previous section.
    pub transform: Transformation,

    /// pixel_size describes the view of the world by the Camera.
    pub pixel_size: f64,

    /// TODO
    pub half_width: f64,

    /// TODO
    pub half_height: f64,
}

impl Camera {
    /// Create a new camera.
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let mut half_width = half_view * aspect;
        let mut half_height = half_view;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Transformation::new(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    /// Compute a ray that starts at the camera and passes through the indicated (x,y) pixel.
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inv = self
            .transform
            .init()
            .inverse(4)
            .expect("Camera transform should be inversable!");
        let pixel = inv * Point::new(world_x, world_y, -1.0);
        let origin = inv * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray { origin, direction }
    }

    /// Render a view of the given world with the camera.
    pub fn render(&self, world: &World) -> Canvas {
        init_progress_bar(self.hsize * self.vsize);
        set_progress_bar_action("Rendering", Color::Blue, Style::Bold);
        let mut canvas = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);

                canvas.write_pixel(x, y, color);
                inc_progress_bar();
            }
        }
        finalize_progress_bar();

        canvas
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{float_eq, Point, Vector, IDENTITY, RGB};
    use std::f64::consts::PI;

    #[test]
    fn construct_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform.init(), IDENTITY);
    }

    #[test]
    fn pixel_size_horizontal_camera() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert!(float_eq(c.pixel_size, 0.01));
    }

    #[test]
    fn pixel_size_vertical_camera() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert!(float_eq(c.pixel_size, 0.01));
    }

    #[test]
    fn ray_center_canvas_camera() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }
    #[test]
    fn ray_corner_canvas_camera() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }
    #[test]
    fn ray_transform_canvas_camera() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = Transformation::new()
            .translation(0.0, -2.0, 5.0)
            .rotate_y(PI / 4.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(2_f64.sqrt() / 2.0, 0.0, -(2_f64.sqrt()) / 2.0)
        );
    }

    #[test]
    fn render_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        c.transform = Transformation::view_transformation(from, to, up);
        let image = c.render(&w);

        assert_eq!(image.pixel_at(5, 5), RGB::new(0.38066, 0.47583, 0.2855));
    }
}
