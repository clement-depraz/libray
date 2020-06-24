use serde::{Deserialize};

use crate::ray_tracing::{Intersectable, Ray};
use super::light::Light;
use super::elements::{Element, Intersection};

#[derive(Deserialize, Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
    pub shadow_bias: f32,
    pub fov: f32
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|element| element.intersect(ray).map(|distance| Intersection::new(distance, element)))
            .min_by(|intersect1, intersect2| intersect1.distance.partial_cmp(&intersect2.distance).unwrap())
    }
}
