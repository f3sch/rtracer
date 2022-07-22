use crate::*;

/// The transformation object describes a general transformation on any object.
/// The abstraction happens since I did not implement the proper tuple as described
/// by the book.
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
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Scales all points of an object.
    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self {
            data: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Rotation around the x axis. Units are in radians.
    pub fn rotate_x(rad: f64) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, rad.cos(), -rad.sin(), 0.0],
                [0.0, rad.sin(), rad.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Rotation around the y axis. Units are in radians.
    pub fn rotate_y(rad: f64) -> Self {
        Self {
            data: [
                [rad.cos(), 0.0, rad.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-rad.sin(), 0.0, rad.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Rotation around the z axis. Units are in radians.
    pub fn rotate_z(rad: f64) -> Self {
        Self {
            data: [
                [rad.cos(), -rad.sin(), 0.0, 0.0],
                [rad.sin(), rad.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Shearing transforms an object in respect to its coordinates.
    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self {
            data: [
                [1.0, xy, xz, 0.0],
                [yx, 1.0, yz, 0.0],
                [zx, zy, 1.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn mul_point_translation() {
        let transform = Transformation::translation(5.0, -3.0, 2.0).init();
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn mul_inv_translation() {
        let transform = Transformation::translation(5.0, -3.0, 2.0).init();
        let inv = transform.inverse(4).unwrap();
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn mul_vec_translation() {
        let transform = Transformation::translation(5.0, -3.0, 2.0).init();
        let v = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn mul_point_scaling() {
        let transform = Transformation::scaling(2.0, 3.0, 4.0).init();
        let p = Point::new(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn mul_vector_scaling() {
        let transform = Transformation::scaling(2.0, 3.0, 4.0).init();
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(transform * v, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn mul_inv_scaling() {
        let transform = Transformation::scaling(2.0, 3.0, 4.0).init();
        let inv = transform.inverse(4).unwrap();
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Vector::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_scaling() {
        let transform = Transformation::scaling(-1.0, 1.0, 1.0).init();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn x_rotate() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::rotate_x(PI / 4.0).init();
        let full_quarter = Transformation::rotate_x(PI / 2.0).init();

        assert_eq!(
            half_quarter * p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn x_inv_rotate() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::rotate_x(PI / 4.0).init();
        let inv = half_quarter.inverse(4).unwrap();

        assert_eq!(
            inv * p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn y_rotate() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Transformation::rotate_y(PI / 4.0).init();
        let full_quarter = Transformation::rotate_y(PI / 2.0).init();

        assert_eq!(
            half_quarter * p,
            Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn z_rotate() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::rotate_z(PI / 4.0).init();
        let full_quarter = Transformation::rotate_z(PI / 2.0).init();

        assert_eq!(
            half_quarter * p,
            Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn xy_shearing(){
        let transform = Transformation::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).init();
        let p = Point::new(2.0,3.0,4.0);

        assert_eq!(transform*p, Point::new(5.0,3.0,4.0));
    }

    #[test]
    fn xz_shearing(){
        let transform = Transformation::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0).init();
        let p = Point::new(2.0,3.0,4.0);

        assert_eq!(transform*p, Point::new(6.0,3.0,4.0));
    }

    #[test]
    fn yx_shearing(){
        let transform = Transformation::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0).init();
        let p = Point::new(2.0,3.0,4.0);

        assert_eq!(transform*p, Point::new(2.0,5.0,4.0));
    }

    #[test]
    fn yz_shearing(){
        let transform = Transformation::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0).init();
        let p = Point::new(2.0,3.0,4.0);

        assert_eq!(transform*p, Point::new(2.0,7.0,4.0));
    }

    #[test]
    fn zx_shearing(){
        let transform = Transformation::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0).init();
        let p = Point::new(2.0,3.0,4.0);

        assert_eq!(transform*p, Point::new(2.0,3.0,6.0));
    }

    #[test]
    fn zy_shearing(){
        let transform = Transformation::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0).init();
        let p = Point::new(2.0,3.0,4.0);

        assert_eq!(transform*p, Point::new(2.0,3.0,7.0));
    }

    #[test]
    fn seq_transform(){
        let p = Point::new(1.0,0.0,1.0);
        let a = Transformation::rotate_x(PI/2.0).init();
        let b = Transformation::scaling(5.0, 5.0, 5.0).init();
        let c = Transformation::translation(10.0, 5.0, 7.0).init();
        let p2 = a*p;
        let p3 = b*p2;
        let p4 = c*p3;

        assert_eq!(p2, Point::new(1.0,-1.0,0.0));
        assert_eq!(p3, Point::new(5.0,-5.0,0.0));
        assert_eq!(p4, Point::new(15.0,0.0,7.0));
    }

    #[test]
    fn chain_transform(){
        let p = Point::new(1.0,0.0,1.0);
        let a = Transformation::rotate_x(PI/2.0).init();
        let b = Transformation::scaling(5.0, 5.0, 5.0).init();
        let c = Transformation::translation(10.0, 5.0, 7.0).init();

        assert_eq!(c*b*a*p, Point::new(15.0,0.0,7.0));
    }
}
