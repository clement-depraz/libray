extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate libray;
extern crate ppm;

use clap::{Arg, App};
use std::fs::{File};
use libray::ray_tracing::scene::Scene;
use std::path::Path;

fn main() {
    let app = App::new("libray")
        .version("0.1")
        .arg(Arg::with_name("scene")
            .required(true)
            .index(1))
        .arg(Arg::with_name("image")
            .required(true)
            .index(2));
    let matches = app.get_matches();

    let scene_path = matches.value_of("scene").unwrap();
    let scene_file = File::open(scene_path).expect("File not found");

    let image_path = Path::new(matches.value_of("image").unwrap());

    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    let image = libray::render(&scene);

    image.save(image_path).unwrap();
}