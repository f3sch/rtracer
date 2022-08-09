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
    set_pattern!(floor, pattern);
    floor.get_material_mut().reflective = 0.1;
    add_object!(world, floor);

    let mut ceiling = Plane::new();
    ceiling.set_transform(Transformation::new().translation(0.0, 10.0, 0.0));
    ceiling.get_material_mut().reflective = 0.1;
    add_object!(world, ceiling);

    let mut checkers = Checkers::checkers_pattern(WHITE, BLACK);
    checkers.set_transform(Transformation::new().translation(10.0, 0.0, 10.0));

    let mut left_wall = Plane::new();
    left_wall.set_transform(
        Transformation::new()
            .rotate_x(PI / 2.0)
            .rotate_y(-PI / 4.0)
            .translation(0.0, 0.0, 10.0),
    );
    set_pattern!(left_wall, checkers);
    add_object!(world, left_wall);

    let mut right_wall = Plane::new();
    right_wall.set_transform(
        Transformation::new()
            .rotate_x(PI / 2.0)
            .rotate_y(PI / 4.0)
            .translation(10.0, 0.0, 0.0),
    );
    set_pattern!(right_wall, checkers);
    add_object!(world, right_wall);

    let mut middle = Cone::new();
    middle.set_cuts(-1.0, 1.0);
    middle.set_closed(false);
    middle.set_transform(Transformation::new().translation(-0.5, 1.0, 0.5));
    middle.get_material_mut().transparency = 1.0;
    middle.get_material_mut().refractive_index = 1.5;
    middle.get_material_mut().ambient = 0.1;
    middle.get_material_mut().diffuse = 0.05;
    add_object!(world, middle);

    let mut middle_back = Cylinder::new();
    middle_back.set_cuts(-1.0, 1.0);
    middle_back.set_closed(true);
    middle_back.set_transform(
        Transformation::new()
            .scaling(0.25, 0.25, 0.25)
            .translation(-0.5, 1.0, -1.0),
    );
    middle_back.set_color(BLACK);
    add_object!(world, middle_back);

    add_object!(world, get_ring(1.5, 0.5));
    add_object!(world, get_ring(1.25, 1.0));
    add_object!(world, get_ring(1.0, 1.5));
    add_object!(world, get_ring(0.75, 2.0));
    let mut middle_ring = get_ring(0.50, 2.5);
    middle_ring.set_closed(true);
    add_object!(world, middle_ring);

    let mut left = Cylinder::new();
    left.set_cuts(-1.0, 1.0);
    left.set_closed(true);
    left.set_transform(
        Transformation::new()
            .scaling(0.33, 0.33, 0.33)
            .translation(-1.5, 0.33, -1.0),
    );
    left.get_material_mut().color = RGB::new(0.0, 0.6, 0.0);
    left.get_material_mut().ambient = 0.8;
    left.get_material_mut().reflective = 0.6;
    left.get_material_mut().refractive_index = 2.417;
    add_object!(world, left);

    world.set_light(PointLight::new(
        Point::new(10.0, 3.5, -10.0),
        RGB::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(1920, 1080, PI / 3.0);

    camera.transform = Transformation::view_transformation(
        Point::new(0.0, 2.0, -6.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let path = Path::new("ch13.ppm");
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

pub fn get_ring(s1: f64, s2: f64) -> Cylinder {
    let mut ring = Cylinder::new();
    ring.set_cuts(-0.25, 0.25);
    ring.set_closed(false);
    ring.set_transform(
        Transformation::new()
            .scaling(s1, s2, s1)
            .translation(1.75, 0.0, -1.25),
    );
    ring.get_material_mut().color = RGB::new(0.8, 0.0, 0.2);

    ring
}
