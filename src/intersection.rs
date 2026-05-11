use crate::color::Color;
use crate::light::lighting;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::world::World;
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Intersection {
    pub t: f32,
    pub object: Sphere,
    pub point: Option<Tuple>,
    pub eye_vector: Option<Tuple>,
    pub normal_vector: Option<Tuple>,
    pub inside: Option<bool>,
    pub over_point: Option<Tuple>,
}

impl Intersection {
    pub fn new(t: f32, object: Sphere) -> Intersection {
        Intersection {
            t,
            object,
            point: None,
            eye_vector: None,
            normal_vector: None,
            inside: None,
            over_point: None,
        }
    }

    pub fn prepare_hit(&mut self, ray: Ray) {
        let point = ray.position(self.t);
        let eye_vector = -ray.direction;
        let mut normal_vector = self.object.normal_at(point);
        let inside;
        if normal_vector.dot(eye_vector) < 0.0 {
            inside = true;
            normal_vector = -normal_vector;
        } else {
            inside = false;
        }
        let over_point = point + (normal_vector * 0.01);
        self.point = Some(point);
        self.eye_vector = Some(eye_vector);
        self.normal_vector = Some(normal_vector);
        self.inside = Some(inside);
        self.over_point = Some(over_point);
    }

    pub fn shade_hit(&self, world: &World) -> Color {
        lighting(
            self.object.material,
            world.light.unwrap(),
            self.point.unwrap(),
            self.eye_vector.unwrap(),
            self.normal_vector.unwrap(),
            world.is_shadowed(self.over_point.unwrap()),
        )
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Intersection) -> Ordering {
        if self.t < other.t {
            Ordering::Less
        } else if self.t > other.t {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Intersection {}

pub fn find_hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    intersections.iter().filter(|i| i.t >= 0.0).min().cloned()
}

#[cfg(test)]
mod tests {
    use crate::EPSILON;
    use crate::color::Color;
    use crate::intersection::{Intersection, find_hit};
    use crate::matrix::Matrix4;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;
    use crate::world::World;

    #[test]
    fn test_an_intersection_encapsulates_t_and_an_object() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn test_aggregating_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let intersections = vec![i1, i2];
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 1.0);
        assert_eq!(intersections[1].t, 2.0);
    }

    #[test]
    fn test_intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = vec![i2, i1];
        let i = find_hit(xs);
        assert!(i.is_some());
        assert_eq!(i.unwrap(), i1);
    }

    #[test]
    fn test_the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = vec![i2, i1];
        let i = find_hit(xs);
        assert!(i.is_some());
        assert_eq!(i.unwrap(), i2);
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = vec![i2, i1];
        let i = find_hit(xs);
        assert!(i.is_none());
    }

    #[test]
    fn test_the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = vec![i1, i2, i3, i4];
        let i = find_hit(xs);
        assert!(i.is_some());
        assert_eq!(i.unwrap(), i4);
    }

    #[test]
    fn test_precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let mut i = find_hit(shape.intersect(r)).unwrap();
        i.prepare_hit(r);
        assert_eq!(i.object, i.object);
        assert_eq!(i.point, Some(Tuple::point(0.0, 0.0, -1.0)));
        assert_eq!(i.eye_vector, Some(Tuple::vector(0.0, 0.0, -1.0)));
        assert_eq!(i.normal_vector, Some(Tuple::vector(0.0, 0.0, -1.0)));
    }

    #[test]
    fn test_the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let mut i = find_hit(shape.intersect(r)).unwrap();
        i.prepare_hit(r);
        assert_eq!(i.inside, Some(false));
    }

    #[test]
    fn test_the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let mut i = find_hit(shape.intersect(r)).unwrap();
        i.prepare_hit(r);
        assert_eq!(i.point, Some(Tuple::point(0.0, 0.0, 1.0)));
        assert_eq!(i.eye_vector, Some(Tuple::vector(0.0, 0.0, -1.0)));
        assert_eq!(i.inside, Some(true));
        // normal would have been (0.0, 0.0, 1.0) but is inverted
        assert_eq!(i.normal_vector, Some(Tuple::vector(0.0, 0.0, -1.0)));
    }

    #[test]
    fn test_shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.objects[0];
        let mut i = find_hit(shape.intersect(r)).unwrap();
        i.prepare_hit(r);
        let c = i.shade_hit(&w);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_the_hit_should_offset_the_point() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut shape = Sphere::default();
        shape.transform = Matrix4::translation(0.0, 0.0, 1.0);
        let mut i = Intersection::new(5.0, shape);
        i.prepare_hit(r);
        assert!(i.over_point.unwrap().z < -EPSILON / 2.0);
        assert!(i.point.unwrap().z > i.over_point.unwrap().z);
    }
}
