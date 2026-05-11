use crate::color::Color;
use crate::intersection::{Intersection, find_hit};
use crate::light::PointLight;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> World {
        World {
            light: None,
            objects: vec![],
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        self.objects
            .iter()
            .flat_map(|object| object.intersect(ray))
            .collect()
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.intersect(ray);
        let hit = find_hit(intersections);
        match hit {
            Some(mut intersection) => {
                intersection.prepare_hit(ray);
                intersection.shade_hit(self)
            }
            None => Color::black(),
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let shadow_vector = self.light.unwrap().position - point;
        let distance = shadow_vector.magnitude();
        let direction = shadow_vector.normalize();
        let shadow_ray = Ray::new(point, direction);
        let intersections = self.intersect(shadow_ray);
        let hit = find_hit(intersections);
        hit.is_some() && hit.unwrap().t < distance
    }
}

impl Default for World {
    fn default() -> World {
        let light =
            PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());

        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::default();
        s2.transform = Matrix4::scaling(0.5, 0.5, 0.5);

        World {
            light: Some(light),
            objects: vec![s1, s2],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::intersection::Intersection;
    use crate::light::PointLight;
    use crate::matrix::Matrix4;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;
    use crate::world::World;

    #[test]
    fn test_creating_a_world() {
        let w = World::new();
        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }

    #[test]
    fn test_the_default_world() {
        let light =
            PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());
        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::default();
        s2.transform = Matrix4::scaling(0.5, 0.5, 0.5);
        let w = World::default();
        assert!(w.light.is_some());
        assert_eq!(w.light.unwrap(), light);
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn test_intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::vector(0.0, 0.0, 1.0),
        );
        let mut xs = w.intersect(r);
        xs.sort();
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn test_the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );
        let c = w.color_at(r);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn test_the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::vector(0.0, 0.0, 1.0),
        );
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let inner = w.objects[1];
        let r = Ray::new(
            Tuple::point(0.0, 0.0, 0.75),
            Tuple::vector(0.0, 0.0, -1.0),
        );
        let c = w.color_at(r);
        assert_eq!(c, inner.material.color);
    }

    #[test]
    fn test_there_is_no_shadow_when_nothing_is_collinear_with_point_and_light()
    {
        let w = World::default();
        let p = Tuple::point(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn test_the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Tuple::point(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(p));
    }

    #[test]
    fn test_there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Tuple::point(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn test_there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Tuple::point(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn test_shade_hit_is_given_an_intersection_in_shadow() {
        let mut world = World::default();
        world.light = Some(PointLight::new(
            Tuple::point(0.0, 0.0, -10.0),
            Color::white(),
        ));
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.transform = Matrix4::translation(0.0, 0.0, 10.0);
        world.objects = vec![s1, s2];
        let r =
            Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut i = Intersection::new(4.0, s2);
        i.prepare_hit(r);
        let c = i.shade_hit(&world);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
