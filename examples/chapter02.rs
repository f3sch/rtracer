use rtracer::*;
use std::{fs::File, io::Write, path::Path};

/// A projectile.
#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

/// An environment.
struct Environment {
    gravity: Vector,
    wind: Vector,
}

/// A tick represents a give projectile in an environment after one unit of time.
fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

fn main() {
    let start = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;

    let p = &mut Projectile {
        position: start,
        velocity,
    };
    let e = &mut Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let width = 900;
    let height = 500;
    let mut c = Canvas::new(width, height);

    loop {
        tick(e, p);
        if p.position.y <= 0.0 {
            break;
        }

        let x = p.position.x as usize;
        let y = height - p.position.y as usize;
        if x <= width - 1 && y <= height - 1 {
            c.write_pixel(x, y, RGB::new(1.0, 0.0, 0.0));
        }
    }

    let path = Path::new("chapter02.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(e) => panic!("Could not create {}: {}", display, e),
        Ok(file) => file,
    };

    let ppm = c.to_ppm();
    match file.write_all(ppm.as_bytes()) {
        Err(e) => panic!("Could not write to {}: {}", display, e),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}
