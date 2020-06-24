use crate::ray_tracing::{Intersectable, Ray};
use super::light::Light;
use super::elements::{Element, Intersection};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
    pub shadow_bias: f32,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
