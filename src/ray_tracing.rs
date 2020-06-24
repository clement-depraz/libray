pub mod elements;
pub mod color;
mod vector;

use vector::{Point, Vector};
use elements::{Scene, Element, Sphere, Plane, Intersection};
use color::Color;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn init_ray(x: u32, y: u32, scene: &Scene) -> Ray {
        let aspect_ratio = (scene.width as f32) / (scene.height as f32);
        let sensor_x = (((x as f32 + 0.5) / scene.width as f32) * 2.0 - 1.0) * aspect_ratio;
        let sensor_y = 1.0 - ((y as f32 + 0.5) / scene.height as f32) * 2.0;

        Ray {
            origin: Point::zero(),
            direction: Vector {
                    x: sensor_x,
                    y: sensor_y,
                    z: -1.0,
                }
                .unit_vector(),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f32>;

    fn surface_normal(&self, hit_point: &Point) -> Vector;
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point),
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let l = self.center - ray.origin;
        let adj2 = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj2 * adj2);
        let radius2 = self.radius * self.radius;

        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj2 - thc;
        let t1 = adj2 + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector {
        (*hit_point - self.center).unit_vector()
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let normal = self.normal.unit_vector();
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: &Point) -> Vector {
        self.normal.unit_vector() * -1.0
    }
}


pub fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);

    let mut color = Color {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
    };
    for light in &scene.lights {
        let direction_to_light = light.direction * -1.0;

        let shadow_ray = Ray {
            origin: hit_point,
            direction: direction_to_light,
        };
        let in_light = scene.trace(&shadow_ray).is_none();

        let light_intensity = if in_light { light.intensity } else { 0.0 };
        let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) *
                          light_intensity;
        let light_reflected = intersection.element.refl_pow() / std::f32::consts::PI;

        let light_color = light.color * light_power * light_reflected;
        color = color + (*intersection.element.color() * light_color);
    }

    color.bound()
}
