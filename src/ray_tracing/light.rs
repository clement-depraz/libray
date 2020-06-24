use super::vector::{Vector, Point};
use super::color::Color;

pub struct PlaneLight {
    pub direction: Vector,
    pub color: Color,
    pub intensity: f32,
}

pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

pub enum Light {
    Plane(PlaneLight),
    Spherical(SphericalLight),
}
impl Light {
    pub fn color(&self) -> Color {
        match *self {
            Light::Plane(ref d) => d.color,
            Light::Spherical(ref s) => s.color,
        }
    }

    pub fn direction_from(&self, hit_point: &Point) -> Vector {
        match *self {
            Light::Plane(ref d) => d.direction * -1.0,
            Light::Spherical(ref s) => (s.position - *hit_point).unit_vector(),
        }
    }

    pub fn intensity(&self, hit_point: &Point) -> f32 {
        match *self {
            Light::Plane(ref d) => d.intensity,
            Light::Spherical(ref s) => {
                let r2 = (s.position - *hit_point).squared_length() as f32;
                s.intensity / (4.0 * ::std::f32::consts::PI * r2)
            }
        }
    }

    pub fn distance(&self, hit_point: &Point) -> f32 {
        match *self {
            Light::Plane(_) => ::std::f32::INFINITY,
            Light::Spherical(ref s) => (s.position - *hit_point).length(),
        }
    }
}   