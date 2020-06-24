extern crate image;

mod ray_tracing;

use ray_tracing::elements::{Scene, Color};
use ray_tracing::{Ray};

use image::{DynamicImage, GenericImage, Rgba, Pixel};

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::init_ray(x, y, scene);

            let intersection = scene.trace(&ray);
            let color = intersection.map(|i| to_rgba(i.element.color())).unwrap_or(black);
            image.put_pixel(x, y, color);
        }
    }
    image
}

fn to_rgba(color: &Color) -> Rgba<u8> {
    Rgba::from_channels(color.red, color.green, color.blue, 0)
}