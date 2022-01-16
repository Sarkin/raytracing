use raytracing::camera;
use raytracing::hittable::get_closest_hit_in_range;
use raytracing::hittable::World;
use raytracing::rand;
use raytracing::ray::Ray;
use raytracing::scene;
use raytracing::vec::Color;

use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::{AtomicUsize, Ordering};

fn ppm_print(img: &[Vec<Color>]) {
    println!("P3\n{} {}\n255", img[0].len(), img.len());
    for row in img {
        for pixel_color in row {
            println!("{}", pixel_color);
        }
    }
}

fn ray_color_blue_gradient(r: Ray) -> Color {
    let u = r.d.unit();
    let t = (u.y + 1.0) / 2.0;
    Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } * (1.0 - t)
        + Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        } * t
}

fn ray_color(r: Ray, w: &World, depth: u32) -> Color {
    if depth == 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    match get_closest_hit_in_range(&w.hit(r), 0.0001, f32::MAX) {
        None => ray_color_blue_gradient(r),
        Some(h) => match h.material.scatter(r, h.hit_record) {
            Some(s) => ray_color(s.scattered_ray, w, depth - 1) * s.attenuation,
            None => Default::default(),
        },
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RenderConfig {
    aspect_ratio: f32,
    img_width: usize,
    img_height: usize,
    number_of_samples: u32,
    depth: u32,
}

fn deserialize_from_file<T: DeserializeOwned>(path: &str) -> T {
    let file = File::open(path).expect("Failed to open file");
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).expect("Failed to deserialize")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene: scene::Scene = deserialize_from_file(args[1].as_str());
    let render_config: RenderConfig = deserialize_from_file(args[2].as_str());

    let mut img = vec![
        vec![
            Color {
                x: 0.0,
                y: 0.0,
                z: 0.0
            };
            render_config.img_width
        ];
        render_config.img_height
    ];

    let cam = camera::Camera::new(scene.camera_config);
    let w = scene.world;

    let c_rows = AtomicUsize::new(0);
    img.par_iter_mut().enumerate().for_each(|(i, row)| {
        for (j, cell) in row.iter_mut().enumerate() {
            let (r, c) = ((render_config.img_height - i - 1) as f32, j as f32);

            for _ in 0..render_config.number_of_samples {
                let u = (r + rand::get_random_offset()) / (render_config.img_height - 1) as f32;
                let v = (c + rand::get_random_offset()) / (render_config.img_width - 1) as f32;
                *cell = *cell + ray_color(cam.get_ray(u, v), &w, render_config.depth);
            }

            *cell = *cell / (render_config.number_of_samples as f32);
        }
        let rows_done = c_rows.fetch_add(1, Ordering::SeqCst);
        eprintln!("Rows remaining {}", render_config.img_height - rows_done);
    });

    eprintln!("Printing image..");
    ppm_print(&img);
    eprintln!("Done!");
}
