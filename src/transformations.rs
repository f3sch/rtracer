use crate::*;
use std::ops::Mul;

/// The transformation object describes a general transformation on any object.
/// The abstraction happens since I did not implement the proper tuple as described
/// by the book.
#[derive(Debug, Clone, Copy)]
pub struct Transformation {
    data: [[f64; 4]; 4],
}

impl Transformation {
    /// Create a new Transformation object.
    pub fn new() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Instantiate the Transformation as a Matrix
    pub fn init(&self) -> Matrix {
        Matrix::new(self.data)
    }

    /// A translation moves a point.
    pub fn translation(self, x: f64, y: f64, z: f64) -> Self {
        let trans = Self {
            data: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        trans * self
    }

    /// Scales all points of an object.
    pub fn scaling(self, x: f64, y: f64, z: f64) -> Self {
        let scale = Self {
            data: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        scale * self
    }

    /// Rotation around the x axis. Units are in radians.
    pub fn rotate_x(self, rad: f64) -> Self {
        let rot = Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, rad.cos(), -rad.sin(), 0.0],
                [0.0, rad.sin(), rad.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        rot * self
    }

    /// Rotation around the y axis. Units are in radians.
    pub fn rotate_y(self, rad: f64) -> Self {
        let rot = Self {
            data: [
                [rad.cos(), 0.0, rad.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-rad.sin(), 0.0, rad.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        rot * self
    }

    /// Rotation around the z axis. Units are in radians.
    pub fn rotate_z(self, rad: f64) -> Self {
        let rot = Self {
            data: [
                [rad.cos(), -rad.sin(), 0.0, 0.0],
                [rad.sin(), rad.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        rot * self
    }

    /// Shearing transforms an object in respect to its coordinates.
    pub fn shearing(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let shear = Self {
            data: [
                [1.0, xy, xz, 0.0],
                [yx, 1.0, yz, 0.0],
                [zx, zy, 1.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        };

        shear * self
    }

    /// Create a Transformation that orients the world relative to the camera.
    pub fn view_transformation(from: Point, to: Point, up: Vector) -> Self {
        let forward = (to - from).normalize();
        let left = forward.cross(up.normalize());
        let true_up = left.cross(forward);
        let orientation = Transformation {
            data: [
                [left.x, left.y, left.z, 0.0],
                [true_up.x, true_up.y, true_up.z, 0.0],
                [-forward.x, -forward.y, -forward.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let translation = Transformation::new().translation(-from.x, -from.y, -from.z);

        orientation * translation
    }
}

impl Mul<Transformation> for Transformation {
    type Output = Transformation;
    fn mul(self, rhs: Transformation) -> Self::Output {
        let data = (self.init() * rhs.init()).get_data();
        Self { data }
    }
}

impl PartialEq for Transformation {
    fn eq(&self, other: &Self) -> bool {
        for r in 0..4 {
            for c in 0..4 {
                if !float_eq(self.data[r][c], other.data[r][c]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Default for Transformation {
    fn default() -> Self {
        Transformation::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn mul_point_translation() {
        let transform = Transformation::new().translation(5.0, -3.0, 2.0).init();
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn mul_inv_translation() {
        let transform = Transformation::new().translation(5.0, -3.0, 2.0).init();
        let inv = transform.inverse(4).unwrap();
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn mul_vec_translation() {
        let transform = Transformation::new().translation(5.0, -3.0, 2.0).init();
        let v = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn mul_point_scaling() {
        let transform = Transformation::new().scaling(2.0, 3.0, 4.0).init();
        let p = Point::new(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn mul_vector_scaling() {
        let transform = Transformation::new().scaling(2.0, 3.0, 4.0).init();
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(transform * v, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn mul_inv_scaling() {
        let transform = Transformation::new().scaling(2.0, 3.0, 4.0).init();
        let inv = transform.inverse(4).unwrap();
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Vector::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_scaling() {
        let transform = Transformation::new().scaling(-1.0, 1.0, 1.0).init();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn x_rotate() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new().rotate_x(PI / 4.0).init();
        let full_quarter = Transformation::new().rotate_x(PI / 2.0).init();

        assert_eq!(
            half_quarter * p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn x_inv_rotate() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new().rotate_x(PI / 4.0).init();
        let inv = half_quarter.inverse(4).unwrap();

        assert_eq!(
            inv * p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt()) / 2.0)
        );
    }

    #[test]
    fn y_rotate() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Transformation::new().rotate_y(PI / 4.0).init();
        let full_quarter = Transformation::new().rotate_y(PI / 2.0).init();

        assert_eq!(
            half_quarter * p,
            Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn z_rotate() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new().rotate_z(PI / 4.0).init();
        let full_quarter = Transformation::new().rotate_z(PI / 2.0).init();

        assert_eq!(
            half_quarter * p,
            Point::new(-(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn xy_shearing() {
        let transform = Transformation::new()
            .shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
            .init();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn xz_shearing() {
        let transform = Transformation::new()
            .shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0)
            .init();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn yx_shearing() {
        let transform = Transformation::new()
            .shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0)
            .init();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn yz_shearing() {
        let transform = Transformation::new()
            .shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
            .init();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn zx_shearing() {
        let transform = Transformation::new()
            .shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
            .init();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn zy_shearing() {
        let transform = Transformation::new()
            .shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
            .init();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn seq_transform() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Transformation::new().rotate_x(PI / 2.0).init();
        let b = Transformation::new().scaling(5.0, 5.0, 5.0).init();
        let c = Transformation::new().translation(10.0, 5.0, 7.0).init();
        let p2 = a * p;
        let p3 = b * p2;
        let p4 = c * p3;

        assert_eq!(p2, Point::new(1.0, -1.0, 0.0));
        assert_eq!(p3, Point::new(5.0, -5.0, 0.0));
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chain_transform() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Transformation::new().rotate_x(PI / 2.0).init();
        let b = Transformation::new().scaling(5.0, 5.0, 5.0).init();
        let c = Transformation::new().translation(10.0, 5.0, 7.0).init();

        assert_eq!(c * b * a * p, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chain_mul_transform() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Transformation::new()
            .rotate_x(PI / 2.0)
            .scaling(5.0, 5.0, 5.0)
            .translation(10.0, 5.0, 7.0)
            .init();

        assert_eq!(a * p, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn default_view_transform() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transformation::view_transformation(from, to, up).init();

        assert_eq!(t, IDENTITY);
    }

    #[test]
    fn pos_z_view_transform() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transformation::view_transformation(from, to, up);

        assert_eq!(t, Transformation::new().scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn move_world_view_transform() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transformation::view_transformation(from, to, up);

        assert_eq!(t, Transformation::new().translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transform() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let t = Transformation::view_transformation(from, to, up);
        let res = Transformation {
            data: [
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.00000],
                [0.00000, 0.00000, 0.00000, 1.00000],
            ],
        };

        assert_eq!(t, res);
    }
}
