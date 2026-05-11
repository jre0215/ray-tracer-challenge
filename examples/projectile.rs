extern crate ray_tracer_challenge;

use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::tuple::Tuple;
use std::fs::File;
use std::io::Write;
use std::path::Path;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Projectile {
        Projectile { position, velocity }
    }
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Environment {
        Environment { gravity, wind }
    }
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;
    Projectile { position, velocity }
}

fn main() {
    let mut projectile = Projectile::new(
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    );
    let environment = Environment::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.01, 0.0, 0.0),
    );
    let mut ticks = 0;

    let mut canvas = Canvas::new(900, 550);
    let color = Color::white();

    loop {
        projectile = tick(&environment, &projectile);
        ticks += 1;
        println!("projectile.position: {:?}", projectile.position);
        let x = projectile.position.x.round() as i32;
        let y = (canvas.height as i32) - (projectile.position.y.round() as i32);
        if x < 0 || x > ((canvas.width - 1) as i32) || y < 0 || y > ((canvas.height - 1) as i32) {
            break;
        }
        canvas.write_pixel(x as usize, y as usize, color);
    }
    println!("ticks: {}", ticks);

    let path = Path::new("projectile.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => {
            panic!("couldn't create {}: {}", display, why)
        }
        Ok(file) => file,
    };

    match file.write_all(canvas.to_ppm().as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display, why)
        }
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
