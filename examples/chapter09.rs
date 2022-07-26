use rtracer::{
    add_object, shapes::Plane, shapes::Shape, shapes::Sphere, Camera, Point, PointLight,
    Transformation, Vector, World, RGB,
};
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    let mut world = World::new();

    let mut floor = Plane::new();
    floor.get_material_mut().color = RGB::new(1.0, 0.9, 0.9);
    floor.get_material_mut().specular = 0.0;
    add_object!(world, floor);

    let mut middle = Sphere::new();
    middle.set_transform(Transformation::new().translation(-0.5, 1.0, 0.5));
    middle.get_material_mut().color = RGB::new(0.1, 1.0, 0.5);
    middle.get_material_mut().diffuse = 0.7;
    middle.get_material_mut().specular = 0.3;
    add_object!(world, middle);

    let mut right = Sphere::new();
    right.set_transform(
        Transformation::new()
            .scaling(0.5, 0.5, 0.5)
            .translation(1.5, 0.5, -0.5),
    );
    right.get_material_mut().color = RGB::new(0.5, 1.0, 0.1);
    right.get_material_mut().diffuse = 0.7;
    right.get_material_mut().specular = 0.3;
    add_object!(world, right);

    let mut left = Sphere::new();
    left.set_transform(
        Transformation::new()
            .scaling(0.33, 0.33, 0.33)
            .translation(-1.5, 0.33, -0.75),
    );
    left.get_material_mut().color = RGB::new(1.0, 0.8, 0.1);
    left.get_material_mut().diffuse = 0.7;
    left.get_material_mut().specular = 0.3;
    add_object!(world, left);

    world.set_light(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        RGB::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(400, 400, PI / 3.0);

    camera.transform = Transformation::view_transformation(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let path = Path::new("ch09.ppm");
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
