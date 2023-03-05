use rs_tracer::base::primative::{Point, Tuple, Vector};

#[derive(Clone, Copy, Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    fn new(position: Point, velocity: Vector) -> Projectile {
        Projectile { position, velocity }
    }
}

#[derive(Clone, Copy)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    fn new(gravity: Vector, wind: Vector) -> Environment {
        Environment { gravity, wind }
    }
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let pos = proj.position + proj.velocity;
    let vel = proj.velocity + env.gravity + env.wind;
    println!("{:?}", proj.position);
    Projectile { position: pos, velocity: vel }
}

fn main() {
    let mut p = Projectile::new(Point::new(0.0, 1.0, 0.0), Vector::new(1.0, 1.0, 0.0));
    let e = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));

    let mut y = p.position.y;

    while y > 0.0 {
        y = p.position.y;
        p = tick(e, p);
    }
}
