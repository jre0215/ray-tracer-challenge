extern crate ray_tracer_challenge;

use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::light::PointLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::matrix::Matrix4;
use ray_tracer_challenge::sphere::Sphere;
use ray_tracer_challenge::tuple::Tuple;
use ray_tracer_challenge::world::World;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut floor = Sphere::default();
    floor.transform = Matrix4::scaling(10.0, 0.01, 10.0);
    floor.material = Material::default();
    floor.material.color = Color::new(0.9, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::default();
    left_wall.transform = Matrix4::translation(0.0, 0.0, 5.0)
        * Matrix4::rotation_y(-FRAC_PI_4)
        * Matrix4::rotation_x(FRAC_PI_2)
        * Matrix4::scaling(10.0, 0.01, 10.0);
    left_wall.material = floor.material;

    let mut right_wall = Sphere::default();
    right_wall.transform = Matrix4::translation(0.0, 0.0, 5.0)
        * Matrix4::rotation_y(FRAC_PI_4)
        * Matrix4::rotation_x(FRAC_PI_2)
        * Matrix4::scaling(10.0, 0.01, 10.0);
    right_wall.material = floor.material;

    let mut middle = Sphere::default();
    middle.transform = Matrix4::translation(-0.5, 1.0, 0.5);
    middle.material = Material::default();
    middle.material.color = Color::new(0.0, 1.0, 0.0);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::default();
    right.transform = Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5);
    right.material = Material::default();
    right.material.color = Color::new(0.0, 0.0, 1.0);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::default();
    left.transform = Matrix4::translation(-1.5, 0.33, -0.75) * Matrix4::scaling(0.33, 0.33, 0.33);
    left.material = Material::default();
    left.material.color = Color::new(1.0, 0.0, 0.0);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::default();
    world.light = Some(PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Color::white(),
    ));
    world.objects = vec![floor, left_wall, right_wall, middle, right, left];

    let mut camera = Camera::new(500, 250, PI / 3.0);
    camera.transform = Matrix4::view_transform(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world);

    let path = Path::new("scene.ppm");
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
