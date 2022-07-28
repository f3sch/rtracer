use crate::{Point, PointLight, Vector, RGB, WHITE,BLACK};
use std::ops::Neg;

/// A Material encapsulates all the properties of the surface.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    /// The color.
    pub color: RGB,

    /// Parameter in Phong reflection model.
    pub ambient: f64,

    /// Parameter in Phong reflection model.
    pub diffuse: f64,

    /// Parameter in Phong reflection model.
    pub specular: f64,

    /// Parameter in Phong reflection model.
    pub shinniness: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color:WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shinniness: 200.0,
        }
    }
}

/// Calculate the lightning of shape from a Light source.
pub fn lightning(
    material: &Material,
    light: &PointLight,
    position: &Point,
    eyev: &Vector,
    normalv: &Vector,
) -> RGB {
    // combine the surface color with the light's color/intensity
    let effective_color = material.color * light.get_intensity();
    let diffuse;
    let specular;
    // find the direction to the light source
    let lightv = (light.get_position() - *position).normalize();
    // compute the ambient contribution
    let ambient = effective_color * material.ambient;
    // light_dot normal represent the cosine of the angle between the
    // light vector and the normal vector.
    // A negative number means the light is on the other side of the surface.
    let light_dot_normal = lightv.dot(*normalv);
    if light_dot_normal <= 0.0 {
        diffuse = RGB::new(0.0, 0.0, 0.0);
        specular = RGB::new(0.0, 0.0, 0.0);
    } else {
        // compute the diffuse contribution
        diffuse = effective_color * material.diffuse * light_dot_normal;
        // reflect_dot_eye represents the cosine of the angle between the
        // reflection vector and the eye vector.
        // A negative number means the light reflects away from the eye.
        let reflectv = lightv.neg().reflect(*normalv);
        let reflect_dot_eye = reflectv.dot(*eyev);

        if reflect_dot_eye <= 0.0 {
            specular = RGB::new(0.0, 0.0, 0.0);
        } else {
            // compute the specular contribution
            let factor = reflect_dot_eye.powf(material.shinniness);
            specular = light.get_intensity() * material.specular * factor;
        }
    }

    // add the three contributin together to get the final shading
    return ambient + diffuse + specular;
}

#[cfg(test)]
mod test {
    use crate::PointLight;

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(m.color, WHITE);
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shinniness, 200.0);
    }

    #[test]
    fn eye_surface_lightning() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let nomralv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), RGB::new(1.0, 1.0, 1.0));
        let result = lightning(&m, &light, &position, &eyev, &nomralv);

        assert_eq!(result, RGB::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn eye_45_surface_lightning() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0);
        let nomralv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), RGB::new(1.0, 1.0, 1.0));
        let result = lightning(&m, &light, &position, &eyev, &nomralv);

        assert_eq!(result, RGB::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn eye_surface_45_lightning() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let nomralv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), RGB::new(1.0, 1.0, 1.0));
        let result = lightning(&m, &light, &position, &eyev, &nomralv);

        assert_eq!(result, RGB::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn eye_surface_path_lightning() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let nomralv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), RGB::new(1.0, 1.0, 1.0));
        let result = lightning(&m, &light, &position, &eyev, &nomralv);

        assert_eq!(result, RGB::new(1.6364, 1.6363, 1.6364));
    }

    #[test]
    fn eye_surface_behind_lightning() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let nomralv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), RGB::new(1.0, 1.0, 1.0));
        let result = lightning(&m, &light, &position, &eyev, &nomralv);

        assert_eq!(result, RGB::new(0.1, 0.1, 0.1));
    }
}
