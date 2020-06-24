extern crate image;

mod ray_tracing;

use ray_tracing::scene::Scene;
use ray_tracing::color::Color;
use ray_tracing::{Ray, get_color};

use image::{DynamicImage, GenericImage, Rgba, Pixel};

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::init_ray(x, y, scene);

            let intersection = scene.trace(&ray);
            let color = intersection.map(|i| to_rgba(&get_color(scene, &ray, &i))).unwrap_or(black);
            image.put_pixel(x, y, color);
        }
    }
    image
}

fn to_rgba(color: &Color) -> Rgba<u8> {
    Rgba::from_channels((color.red / 255.0) as u8, (color.green / 255.0) as u8, (color.blue / 255.0) as u8, 0)
}