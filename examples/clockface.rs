extern crate ray_tracer_challenge;

use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::matrix::Matrix4;
use ray_tracer_challenge::tuple::Tuple;
use std::f32::consts::{FRAC_PI_6, PI};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Draws a simple clock face using matrix transformations.
fn main() {
    let mut canvas = Canvas::new(500, 500);
    let color = Color::white();

    // Start at 12 o'clock
    let mut hour_point = Tuple::point(0.0, 1.0, 0.0);

    // Rotate 1/12 of a circle for each hour
    let hour_rotation = Matrix4::rotation_z(-FRAC_PI_6);

    // Move the clock face to the center of the image
    let translation = Matrix4::translation(
        (canvas.width as f32) / 2.0,
        (canvas.height as f32) / 2.0,
        0.0,
    );

    // Make the clock face larger
    let clock_radius = (3.0 * canvas.width as f32) / 8.0;
    let scaling = Matrix4::scaling(clock_radius, clock_radius, 0.0);

    // Flip the clock face horizontally since the y-axis is inverted on
    // the canvas (i.e., y increases going *down* the canvas)
    let rotation_x = Matrix4::rotation_x(PI);

    // Apply all the transformations at once
    let transform = translation * scaling * rotation_x;

    for _ in 0..12 {
        let transformed_point = transform * hour_point;
        dbg!(hour_point);
        dbg!(transformed_point);
        canvas.write_pixel(
            transformed_point.x.round() as usize,
            transformed_point.y.round() as usize,
            color,
        );
        hour_point = hour_rotation * hour_point;
    }

    let path = Path::new("clockface.ppm");
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
