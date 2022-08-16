use rtracer::{shapes::Sphere, Canvas, Point, PointLight, Ray, Shape, Transformation, RGB, WHITE};
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    // build world
    let mut shape = Sphere::new();
    shape.set_color(RGB::new(1.0, 0.2, 1.0));
    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = WHITE;
    let light = PointLight::new(light_position, light_color);

    // draw basic
    draw_shape(&shape, &light, "ch06_circle.ppm");

    // shrink it along the y axis
    let t = Transformation::new().scaling(1.0, 0.5, 1.0);
    shape.set_transform(t);
    draw_shape(&shape, &light, "ch06_shrink_y.ppm");

    // shrink it along the x axis
    let t = Transformation::new().scaling(0.5, 1.0, 1.0);
    shape.set_transform(t);
    draw_shape(&shape, &light, "ch06_shrink_x.ppm");

    // shrink it and rotate it!
    let t = Transformation::new()
        .scaling(0.5, 1.0, 1.0)
        .rotate_z(PI / 4.0);
    shape.set_transform(t);
    draw_shape(&shape, &light, "ch06_shrink_rotate.ppm");

    // shrink it and skew it!
    let t = Transformation::new()
        .scaling(0.5, 1.0, 1.0)
        .shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    shape.set_transform(t);
    draw_shape(&shape, &light, "ch06_shrink_skew.ppm");
}

fn draw_shape(shape: &dyn Shape, light: &PointLight, file_name: &str) {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    let wall_size = 7.0;
    let canvas_pixels = 400;

    let pixel_size = wall_size / canvas_pixels as f64;

    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&r);

            if xs.is_some() {
                let xs = xs.unwrap();
                let point = r.position(xs[0].t);
                let normal = xs[0].object.normal_at(point, None);
                let eye = -r.direction;
                let color = xs[0]
                    .object
                    .get_material()
                    .lightning(shape, *light, point, eye, normal, false);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    write_file(file_name, canvas.to_ppm().as_bytes())
}

fn write_file(file_name: &str, ppm: &[u8]) {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(ppm) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    };
}
