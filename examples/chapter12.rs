use rtracer::*;
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    let mut world = World::new();

    let mut checkers = Checkers::checkers_pattern(WHITE, BLACK);
    checkers.set_transform(Transformation::new().scaling(0.15, 0.15, 0.15));

    let mut stripes1 = Stripes::stripe_pattern(RGB::from_u8(161, 64, 5), RGB::from_u8(145, 41, 3));
    stripes1.set_transform(
        Transformation::new()
            .scaling(0.05, 0.05, 0.05)
            .rotate_y(PI / 2.0),
    );

    let mut stripes2 = Stripes::stripe_pattern(RGB::from_u8(161, 64, 5), RGB::from_u8(145, 41, 3));
    stripes2.set_transform(Transformation::new().scaling(0.05, 0.05, 0.05));

    let mut floor = Cube::new();
    floor.set_transform(
        Transformation::new()
            .scaling(5.0, 0.1, 5.0)
            .translation(0.0, -0.1, 0.0),
    );
    floor.get_material_mut().pattern = Some(Box::new(checkers));
    add_object!(world, floor);

    let mut right_wall = Cube::new();
    right_wall.set_transform(
        Transformation::new()
            .scaling(0.1, 4.0, 5.0)
            .translation(-5.1, 4.0, 0.0),
    );
    right_wall.get_material_mut().pattern = Some(Box::new(stripes1));
    add_object!(world, right_wall);

    let mut left_wall = Cube::new();
    left_wall.set_transform(
        Transformation::new()
            .scaling(0.1, 4.0, 5.0)
            .translation(5.1, 4.0, 0.0),
    );
    left_wall.get_material_mut().pattern = Some(Box::new(stripes1));
    add_object!(world, left_wall);

    let mut back_wall = Cube::new();
    back_wall.set_transform(
        Transformation::new()
            .scaling(5.0, 4.0, 0.1)
            .translation(0.0, 4.0, 5.1),
    );
    back_wall.get_material_mut().pattern = Some(Box::new(stripes2));
    add_object!(world, back_wall);

    let mut painting = Cube::new();
    painting.set_transform(
        Transformation::new()
            .scaling(1.0, 2.0, 0.1)
            .translation(-1.5, 4.0, 4.9),
    );
    painting.set_color(RGB::new(0.1, 1.0, 0.1));
    add_object!(world, painting);

    let mut painting2 = Cube::new();
    painting2.set_transform(
        Transformation::new()
            .scaling(1.75, 0.5, 0.1)
            .translation(1.5, 4.0, 4.9),
    );
    painting2.set_color(RGB::new(1.0, 0.3, 0.3));
    add_object!(world, painting2);

    let mut painting3 = Cube::new();
    painting3.set_transform(
        Transformation::new()
            .scaling(1.75, 0.5, 0.1)
            .translation(1.5, 2.75, 4.9),
    );
    painting3.set_color(RGB::new(0.0, 0.3, 1.0));
    add_object!(world, painting3);

    let mut mirror = Cube::new();
    mirror.set_transform(
        Transformation::new()
            .scaling(0.01, 2.0, 4.0)
            .translation(5.0, 3.0, 0.0),
    );
    mirror.get_material_mut().reflective = 1.0;
    mirror.get_material_mut().refractive_index = 1.458;
    add_object!(world, mirror);

    let mut table_top = Cube::new();
    table_top.set_transform(
        Transformation::new()
            .scaling(2.5, 0.1, 3.0)
            .translation(0.5, 1.25, 0.0),
    );
    table_top.get_material_mut().pattern = Some(Box::new(stripes1));
    table_top.get_material_mut().reflective = 0.02;
    table_top.get_material_mut().refractive_index = 3.45;
    add_object!(world, table_top);

    let mut leg1 = Cube::new();
    leg1.set_transform(
        Transformation::new()
            .scaling(0.1, 0.65, 0.1)
            .translation(-1.9, 0.65, -2.9),
    );
    leg1.set_color(RGB::from_u8(161, 64, 5));
    add_object!(world, leg1);

    let mut leg2 = Cube::new();
    leg2.set_transform(
        Transformation::new()
            .scaling(0.1, 0.65, 0.1)
            .translation(2.9, 0.65, -2.9),
    );
    leg2.set_color(RGB::from_u8(161, 64, 5));
    add_object!(world, leg2);

    let mut leg3 = Cube::new();
    leg3.set_transform(
        Transformation::new()
            .scaling(0.1, 0.65, 0.1)
            .translation(2.9, 0.65, 2.9),
    );
    leg3.set_color(RGB::from_u8(161, 64, 5));
    add_object!(world, leg3);

    let mut leg4 = Cube::new();
    leg4.set_transform(
        Transformation::new()
            .scaling(0.1, 0.65, 0.1)
            .translation(-1.9, 0.65, 2.9),
    );
    leg4.set_color(RGB::from_u8(161, 64, 5));
    add_object!(world, leg4);

    let mut glass_block = Cube::new();
    glass_block.set_transform(
        Transformation::new()
            .scaling(0.1, 1.0, 1.0)
            .translation(-0.75, 2.35, -1.0),
    );
    glass_block.get_material_mut().color = RGB::from_u8(211, 102, 151);
    glass_block.get_material_mut().transparency = 1.0;
    add_object!(world, glass_block);

    let mut block1 = Cube::new();
    block1.set_transform(
        Transformation::new()
            .scaling(0.1, 0.1, 0.1)
            .translation(0.5, 1.45, -2.0),
    );
    block1.set_color(RGB::from_u8(213, 14, 151));
    add_object!(world, block1);

    let mut block3 = Cube::new();
    block3.set_transform(
        Transformation::new()
            .scaling(0.2, 0.2, 0.2)
            .translation(1.75, 1.55, -1.0),
    );
    block3.set_color(RGB::from_u8(10, 234, 36));
    add_object!(world, block3);

    let mut block4 = Cube::new();
    block4.set_transform(
        Transformation::new()
            .scaling(0.55, 0.5, 1.75)
            .translation(0.2, 1.55, 0.05),
    );
    block4.get_material_mut().reflective = 0.6;
    block4.get_material_mut().refractive_index = 1.31;
    block4.get_material_mut().ambient = 0.025;
    block4.get_material_mut().diffuse = 0.25;
    block4.get_material_mut().color = RGB::from_u8(237, 234, 36);
    add_object!(world, block4);

    world.set_light(PointLight::new(
        Point::new(3.0, 11.0, -10.0),
        RGB::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(1280, 1280, PI / 3.0);

    camera.transform = Transformation::view_transformation(
        Point::new(-4.0, 2.5, -4.8),
        Point::new(0.90, 1.25, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let path = Path::new("ch12.ppm");
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
