extern crate ppm;
extern crate rayon;

pub mod ray_tracing;

use ray_tracing::scene::Scene;
use ray_tracing::{Ray, get_color};

use rayon::prelude::*;
use ppm::{Image, PPMType, Pixel};

pub fn render(scene: &Scene) -> Image {
    let zero_vec = vec![Pixel::new(0, 0, 0); (scene.width * scene.height) as usize];

    let black = Pixel::new(0, 0, 0);
    let pixel_vec: Vec<Pixel> = zero_vec.par_iter().enumerate().map(|(index, _)| {
        let x = index as u32 % scene.width;
        let y = index as u32 / scene.width;
        let ray = Ray::init_ray(x, y, scene);

        let intersection = scene.trace(&ray);
        intersection.map(|i| {
            let color = &get_color(scene, &ray, &i);
            Pixel::new(
                (color.red * 255.0) as u8,
                (color.green * 255.0) as u8,
                (color.blue * 255.0) as u8)
        }).unwrap_or(black)
    }).collect();

    Image::new(PPMType::P3,
        scene.height as usize,
        scene.width as usize,
        255,
        pixel_vec)
}
