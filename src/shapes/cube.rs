use crate::*;
use uuid::Uuid;

/// Cube.
#[derive(Debug)]
pub struct Cube {
    /// Unique id.
    uuid: Uuid,

    /// Transformation matrix
    transform: Transformation,

    /// The material of a sphere
    material: Material,

    /// Parent id
    parent: Option<Uuid>,
}

impl Cube {
    /// Create a new sphere.
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            transform: Transformation::new(),
            material: Material::default(),
            parent: None,
        }
    }

    pub fn set_color(&mut self, color: RGB) {
        self.material.color = color;
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let tmin = tmin_numerator / direction;
    let tmax = tmax_numerator / direction;

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl Shape for Cube {
    fn id(&self) -> Uuid {
        self.uuid
    }

    fn parent_id(&self) -> Option<Uuid> {
        self.parent
    }

    fn set_parent_id(&mut self, id: Uuid) {
        self.parent = Some(id);
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    fn get_transform(&self) -> Transformation {
        self.transform
    }

    fn set_transform(&mut self, t: Transformation) {
        self.transform = t;
    }

    fn local_intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

        let min_values = [xtmin, ytmin, ztmin];
        let tmin = min_values.iter().max_by(|x, y| float_cmp(**x, **y));
        let max_values = [xtmax, ytmax, ztmax];
        let tmax = max_values.iter().min_by(|x, y| float_cmp(**x, **y));

        let tmin = *tmin.unwrap();
        let tmax = *tmax.unwrap();

        if tmin > tmax {
            None
        } else {
            Some(vec![
                Intersection::new(tmin, self),
                Intersection::new(tmax, self),
            ])
        }
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        let max_values = [point.x.abs(), point.y.abs(), point.z.abs()];
        let maxc = max_values.iter().max_by(|x, y| float_cmp(**x, **y));

        let maxc = *maxc.unwrap();

        if (maxc - point.x.abs()).abs() < EPSILON {
            Vector::new(point.x, 0.0, 0.0)
        } else if (maxc - point.y.abs()).abs() < EPSILON {
            Vector::new(0.0, point.y, 0.0)
        } else {
            Vector::new(0.0, 0.0, point.z)
        }
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_cube() {
        let c = Cube::new();
        let rs = vec![
            Ray::new(Point::new(5.0, 0.5, 0.0), Vector::new(-1.0, 0.0, 0.0)),
            Ray::new(Point::new(-5.0, 0.5, 0.0), Vector::new(1.0, 0.0, 0.0)),
            Ray::new(Point::new(0.5, 5.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            Ray::new(Point::new(0.5, -5.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            Ray::new(Point::new(0.5, 0.0, 5.0), Vector::new(0.0, 0.0, -1.0)),
            Ray::new(Point::new(0.5, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0)),
            Ray::new(Point::new(0.0, 0.5, 0.0), Vector::new(0.0, 0.0, 1.0)),
        ];
        let xss = [
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (-1.0, 1.0),
        ];
        assert_eq!(rs.len(), xss.len());

        for i in 0..rs.len() {
            let r = rs[i];
            let xs_expect = xss[i];
            let xs = c.local_intersect(&r);
            assert!(xs.is_some());
            let xs = xs.unwrap();

            assert!(float_eq(xs[0].t, xs_expect.0));
            assert!(float_eq(xs[1].t, xs_expect.1));
        }
    }

    #[test]
    fn ray_miss_cube() {
        let c = Cube::new();
        let data = vec![
            (
                Point::new(-2.0, 0.0, 0.0),
                Vector::new(0.2673, 0.5345, 0.8018),
            ),
            (
                Point::new(0.0, -2.0, 0.0),
                Vector::new(0.8018, 0.2673, 0.5345),
            ),
            (
                Point::new(0.0, 0.0, -2.0),
                Vector::new(0.5345, 0.8018, 0.2673),
            ),
            (Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for rec in data {
            let r = Ray::new(rec.0, rec.1);
            let xs = c.local_intersect(&r);
            assert_eq!(None, xs);
        }
    }

    #[test]
    fn normal_cube() {
        let c = Cube::new();
        let data = vec![
            (Point::new(1.0, 0.5, -0.8), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -0.2, 0.9), Vector::new(-1.0, 0.0, 0.0)),
            (Point::new(-0.4, 1.0, -0.1), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.3, -1.0, -0.7), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(-0.6, 0.3, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(0.4, 0.4, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -1.0, -1.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for rec in data {
            let p = rec.0;
            let normal = c.local_normal_at(p);
            assert_eq!(rec.1, normal);
        }
    }
}
