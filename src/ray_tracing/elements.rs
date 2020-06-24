use serde::{Deserialize};

use super::vector::{Point, Vector};
use super::color::Color;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub color: Color,
    pub refl_pow: f32,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Plane {
    pub origin: Point,
    #[serde(deserialize_with = "Vector::deserialize_as_norm")]
    pub normal: Vector,
    pub color: Color,
    pub refl_pow: f32,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &s.color,
            Element::Plane(ref p) => &p.color,
        }
    }    
    
    pub fn refl_pow(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.refl_pow,
            Element::Plane(ref p) => p.refl_pow,
        }
    }
}

pub struct Intersection<'a> {
    pub distance: f32,
    pub element: &'a Element,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f32, element: &'b Element) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have a finite distance.");
        }
        Intersection {
            distance: distance,
            element: element
        }
    }
}
