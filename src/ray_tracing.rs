pub mod elements;
mod vector;

use vector::{Point, Vector};
use elements::{Scene, Sphere};

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
}
