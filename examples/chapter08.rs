use rtracer::{
    add_object, shapes::Sphere, Camera, Point, PointLight, Shape, Transformation, Vector, World,
    RGB,
};
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    let mut world = World::new();

    let mut right_wall = Sphere::new();
    right_wall.set_transform(
        Transformation::new()
            .scaling(40.0, 40.0, 40.0)
            .rotate_x(PI / 2.0)
            .rotate_y(PI)
            .translation(0.0, 0.0, 10.0),
    );
    right_wall.get_material_mut().color = RGB::new(1.0, 0.9, 0.9);
    right_wall.get_material_mut().specular = 0.0;
    add_object!(world, right_wall);

    let mut wrist = Sphere::new();
    wrist.set_transform(
        Transformation::new()
            .scaling(0.5, 0.5, 0.5)
            .translation(-1.25, 1.5, -2.0),
    );
    wrist.get_material_mut().color = RGB::new(0.1, 1.0, 0.5);
    wrist.get_material().diffuse = 0.7;
    wrist.get_material().specular = 0.3;
    add_object!(world, wrist);

    let mut palm = Sphere::new();
    palm.set_transform(
        Transformation::new()
            .scaling(0.75, 0.75, 0.75)
            .translation(-0.5, 2.0, -1.75),
    );
    palm.get_material_mut().color = RGB::new(0.1, 1.0, 0.5);
    palm.get_material_mut().diffuse = 0.7;
    palm.get_material_mut().specular = 0.3;
    add_object!(world, palm);

    let mut finger1 = Sphere::new();
    finger1.set_transform(
        Transformation::new()
            .scaling(1.0, 0.2, 0.1)
            .translation(0.0, 2.0, -2.5),
    );
    finger1.get_material_mut().color = RGB::new(1.0, 0.8, 0.1);
    finger1.get_material_mut().diffuse = 0.7;
    finger1.get_material_mut().specular = 0.3;
    add_object!(world, finger1);

    let mut finger2 = Sphere::new();
    finger2.set_transform(
        Transformation::new()
            .scaling(1.0, 0.2, 0.1)
            .translation(0.0, 1.80, -2.5),
    );
    finger2.get_material_mut().color = RGB::new(1.0, 0.8, 0.1);
    finger2.get_material_mut().diffuse = 0.7;
    finger2.get_material_mut().specular = 0.3;
    add_object!(world, finger2);

    let mut finger3 = Sphere::new();
    finger3.set_transform(
        Transformation::new()
            .scaling(0.75, 0.2, 0.1)
            .translation(0.0, 1.60, -2.5),
    );
    finger3.get_material_mut().color = RGB::new(1.0, 0.8, 0.1);
    finger3.get_material_mut().diffuse = 0.7;
    finger3.get_material_mut().specular = 0.3;
    add_object!(world, finger3);

    let mut finger4 = Sphere::new();
    finger4.set_transform(
        Transformation::new()
            .scaling(0.75, 0.2, 0.1)
            .rotate_z(PI / 2.0)
            .translation(-0.75, 2.40, -2.5),
    );
    finger4.get_material_mut().color = RGB::new(1.0, 0.8, 0.1);
    finger4.get_material_mut().diffuse = 0.7;
    finger4.get_material_mut().specular = 0.3;
    add_object!(world, finger4);

    world.set_light(PointLight::new(
        Point::new(-2.0, 1.0, -10.0),
        RGB::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(400, 400, PI / 2.5);

    camera.transform = Transformation::view_transformation(
        Point::new(1.25, 1.0, -6.0),
        Point::new(0.2, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let path = Path::new("chapter_08.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let ppm = canvas.to_ppm();
    match file.write_all(ppm.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    };
}
