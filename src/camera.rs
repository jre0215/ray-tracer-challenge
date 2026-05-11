use crate::canvas::Canvas;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::world::World;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f32,
    pub transform: Matrix4,
    pub half_width: f32,
    pub half_height: f32,
    pub pixel_size: f32,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (hsize as f32) / (vsize as f32);
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / (hsize as f32);
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix4::identity(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    fn ray_for_pixel(
        &self,
        inverse_transform: Matrix4,
        origin: Tuple,
        px: usize,
        py: usize,
    ) -> Ray {
        let x_offset = ((px as f32) + 0.5) * self.pixel_size;
        let y_offset = ((py as f32) + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = inverse_transform * Tuple::point(world_x, world_y, -1.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);

        let inverse_transform = self.transform.inverse();
        let origin = inverse_transform * Tuple::point(0.0, 0.0, 0.0);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(inverse_transform, origin, x, y);
                let color = world.color_at(ray);
                canvas.write_pixel(x, y, color);
            }
        }
        canvas
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::color::Color;
    use crate::matrix::Matrix4;
    use crate::tuple::Tuple;
    use crate::world::World;
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};

    #[test]
    fn test_constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = FRAC_PI_2;
        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, FRAC_PI_2);
        assert_eq!(c.transform, Matrix4::identity());
    }

    #[test]
    fn test_the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, FRAC_PI_2);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn test_the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, FRAC_PI_2);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn test_constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let inverse_transform = c.transform.inverse();
        let origin = inverse_transform * Tuple::point(0.0, 0.0, 0.0);
        let r = c.ray_for_pixel(inverse_transform, origin, 100, 50);
        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let inverse_transform = c.transform.inverse();
        let origin = inverse_transform * Tuple::point(0.0, 0.0, 0.0);
        let r = c.ray_for_pixel(c.transform.inverse(), origin, 0, 0);
        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn test_constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, FRAC_PI_2);
        c.transform = Matrix4::rotation_y(FRAC_PI_4) * Matrix4::translation(0.0, -2.0, 5.0);
        let inverse_transform = c.transform.inverse();
        let origin = inverse_transform * Tuple::point(0.0, 0.0, 0.0);
        let r = c.ray_for_pixel(inverse_transform, origin, 100, 50);
        assert_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
        assert_eq!(r.direction, Tuple::vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0));
    }

    #[test]
    fn test_rendering_a_world_with_a_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, FRAC_PI_2);
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        c.transform = Matrix4::view_transform(from, to, up);
        let image = c.render(w);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
