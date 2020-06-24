use super::vector::Point;
use crate::ray_tracing::{Intersectable, Ray};

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub spheres: Vec<Sphere>,
}

pub struct Intersection<'a> {
    pub distance: f32,
    pub object: &'a Sphere,
}
impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f32, object: &'b Sphere) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have a finite distance.");
        }
        Intersection {
            distance: distance,
            object: object
        }
    }
}
impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.spheres
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
