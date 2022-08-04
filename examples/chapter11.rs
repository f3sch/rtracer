use rtracer::*;
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    let mut world = World::new();

    let mut pattern = Checkers::checkers_pattern(WHITE, BLACK);
    pattern.set_transform(
        Transformation::new()
            .scaling(0.1, 0.1, 0.1)
            .rotate_y(0.174)
            .translation(10.0, 0.0, 10.0),
    );

    let mut floor = Plane::new();
    floor.get_material_mut().pattern = Some(Box::new(pattern));
    floor.get_material_mut().reflective = 0.1;
    world.add_object(Box::new(floor));

    let mut ceiling = Plane::new();
    ceiling.set_transform(Transformation::new().translation(0.0, 10.0, 0.0));
    ceiling.get_material_mut().reflective = 0.1;
    ceiling.get_material_mut().pattern = Some(Box::new(pattern));
    world.add_object(Box::new(ceiling));

    let mut checkers = Checkers::checkers_pattern(WHITE, BLACK);
    checkers.set_transform(Transformation::new().translation(10.0, 0.0, 10.0));

    let mut left_wall = Plane::new();
    left_wall.set_transform(
        Transformation::new()
            .rotate_x(PI / 2.0)
            .rotate_y(-PI / 4.0)
            .translation(0.0, 0.0, 10.0),
    );
    left_wall.get_material_mut().pattern = Some(Box::new(checkers));
    world.add_object(Box::new(left_wall));

    let mut right_wall = Plane::new();
    right_wall.set_transform(
        Transformation::new()
            .rotate_x(PI / 2.0)
            .rotate_y(PI / 4.0)
            .translation(10.0, 0.0, 0.0),
    );
    right_wall.get_material_mut().pattern = Some(Box::new(checkers));
    world.add_object(Box::new(right_wall));

    let mut middle = Sphere::new();
    middle.set_transform(Transformation::new().translation(-0.5, 1.0, 0.5));
    middle.get_material_mut().transparency = 1.0;
    middle.get_material_mut().refractive_index = 1.5;
    middle.get_material_mut().ambient = 0.1;
    middle.get_material_mut().diffuse = 0.05;
    world.add_object(Box::new(middle));

    let mut middle_back = Sphere::new();
    middle_back.set_transform(
        Transformation::new()
            .scaling(0.25, 0.25, 0.25)
            .translation(-0.5, 1.0, -1.0),
    );
    middle_back.get_material_mut().color = BLACK;
    world.add_object(Box::new(middle_back));

    let mut right = Sphere::new();
    right.set_transform(
        Transformation::new()
            .scaling(0.5, 0.5, 0.5)
            .translation(1.5, 0.5, -0.5),
    );
    right.get_material_mut().color = RED;
    right.get_material_mut().ambient = 0.5;
    right.get_material_mut().reflective = 0.25;
    world.add_object(Box::new(right));

    let mut left = Sphere::new();
    left.set_transform(
        Transformation::new()
            .scaling(0.33, 0.33, 0.33)
            .translation(-1.5, 0.33, -0.75),
    );
    left.get_material_mut().color = RGB::new(0.0, 0.6, 0.0);
    left.get_material_mut().ambient = 0.8;
    left.get_material_mut().reflective = 0.6;
    left.get_material_mut().refractive_index = 2.417;
    world.add_object(Box::new(left));

    world.set_light(PointLight::new(
        Point::new(10.0, 3.5, -10.0),
        RGB::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(614, 614, PI / 3.0);

    camera.transform = Transformation::view_transformation(
        Point::new(0.0, 1.5, -4.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let path = Path::new("ch11.ppm");
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
