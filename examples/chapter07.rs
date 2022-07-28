use rtracer::*;
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    // build world
    let mut world = World::new();

    let mut floor = Sphere::new();
    floor.set_transform(Transformation::new().scaling(10.0, 0.01, 10.0));
    let mut m = Material::default();
    m.color = RGB::new(1.0, 0.9, 0.9);
    m.specular = 0.0;
    floor.set_material(m);
    add_object!(world, floor);

    let mut left_wall = Sphere::new();
    left_wall.set_transform(
        Transformation::new()
            .scaling(10.0, 0.01, 10.0)
            .rotate_x(PI / 2.0)
            .rotate_y(-PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    left_wall.set_material(m);
    add_object!(world, left_wall);

    let mut right_wall = Sphere::new();
    right_wall.set_transform(
        Transformation::new()
            .scaling(10.0, 0.01, 10.0)
            .rotate_x(PI / 2.0)
            .rotate_y(PI / 4.0)
            .translation(0.0, 0.0, 5.0),
    );
    right_wall.set_material(m);
    add_object!(world, right_wall);

    let mut middle = Sphere::new();
    middle.set_transform(Transformation::new().translation(-0.5, 1.0, 0.5));
    let mut m = Material::default();
    m.color = RGB::new(0.1, 1.0, 0.5);
    m.diffuse = 0.7;
    m.specular = 0.3;
    middle.set_material(m);
    add_object!(world, middle);

    let mut right = Sphere::new();
    right.set_transform(
        Transformation::new()
            .scaling(0.5, 0.5, 0.5)
            .translation(1.5, 0.5, -0.5),
    );
    let mut m = Material::default();
    m.color = RGB::new(0.5, 1.0, 0.1);
    m.diffuse = 0.7;
    m.specular = 0.3;
    right.set_material(m);
    add_object!(world, right);

    let mut left = Sphere::new();
    left.set_transform(
        Transformation::new()
            .scaling(0.33, 0.33, 0.33)
            .translation(-1.5, 0.33, -0.75),
    );
    let mut m = Material::default();
    m.color = RGB::new(1.0, 0.8, 0.1);
    m.diffuse = 0.7;
    m.specular = 0.3;
    left.set_material(m);
    add_object!(world, left);

    world.set_light(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        RGB::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(100, 50, PI / 3.0);
    camera.transform = Transformation::view_transformation(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let path = Path::new("ch07.ppm");
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
