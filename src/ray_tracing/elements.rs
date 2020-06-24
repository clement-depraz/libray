use super::vector::{Point, Vector};
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

pub struct Plane {
    pub origin: Point,
    pub normal: Vector,
    pub color: Color,
}

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
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub elements: Vec<Element>,
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
impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
