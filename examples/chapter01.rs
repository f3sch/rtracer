use rtracer::{Point, Vector};

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
fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };

    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    loop {
        p = tick(&e, p);
        println!("{:?}", p);
        if p.position.y <= 0.0 {
            break;
        }
    }
}
