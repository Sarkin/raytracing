mod camera;
mod hittable;
mod rand;
mod ray;
mod sphere;
mod vec;

use hittable::get_closest_hit_in_range;
use hittable::Dielectric;
use hittable::Lambertian;
use hittable::Material;
use hittable::Metal;
use hittable::Object;
use hittable::World;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use std::sync::atomic::{AtomicUsize, Ordering};
use vec::Color;
use vec::Point;
use vec::Vec3;

use serde::{Deserialize, Serialize};

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

fn get_world() -> World {
    let mut w: World = Default::default();
    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: Point {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            r: 0.5,
        }),
        material: Box::new(Lambertian {
            albedo: Color {
                x: 0.1,
                y: 0.2,
                z: 0.3,
            },
        }),
    });
    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: Point {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            r: 0.5,
        }),
        material: Box::new(Dielectric { ir: 1.5 }),
    });
    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: Point {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            r: 0.5,
        }),
        material: Box::new(Metal {
            albedo: Color {
                x: 0.8,
                y: 0.8,
                z: 0.8,
            },
            fuzziness: 0.0,
        }),
    });
    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: Point {
                x: 0.0,
                y: -500.5,
                z: -1.0,
            },
            r: 500.0,
        }),
        material: Box::new(Lambertian {
            albedo: Color {
                x: 0.1,
                y: 0.7,
                z: 0.3,
            },
        }),
    });
    w
}

fn generate_world() -> World {
    let mut w: World = Default::default();

    let earth_radius = 500.0;
    let earth_center = Point {
        x: 0.0,
        y: -earth_radius,
        z: 0.0,
    };

    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: earth_center,
            r: earth_radius,
        }),
        material: Box::new(Lambertian {
            albedo: Color {
                x: 0.8,
                y: 0.8,
                z: 0.8,
            },
        }),
    });

    for xi in -10..10 {
        for zi in -10..10 {
            let (x, z) = (
                (xi * 3) as f32 + rand::get_random_offset(),
                (zi * 3) as f32 + rand::get_random_offset(),
            );
            let r = 0.5;
            let c =
                (Point { x, y: 0.0, z } - earth_center).unit() * (earth_radius + r) + earth_center;

            let material_p = rand::get_random_float();
            let material: Box<dyn Material + Sync>;
            if material_p > 0.8 {
                material = Box::new(Metal {
                    albedo: Color {
                        x: rand::get_random_float(),
                        y: rand::get_random_float(),
                        z: rand::get_random_float(),
                    },
                    fuzziness: 0.1 * rand::get_random_float(),
                });
            } else if material_p > 0.7 {
                material = Box::new(Dielectric { ir: 1.5 });
            } else {
                material = Box::new(Lambertian {
                    albedo: Color {
                        x: rand::get_random_float(),
                        y: rand::get_random_float(),
                        z: rand::get_random_float(),
                    },
                });
            }
            w.add_object(Object {
                hittable: Box::new(Sphere { o: c, r }),
                material,
            });
        }
    }

    w
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

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    render_config: RenderConfig,
    camera_config: camera::CameraConfig,
}

use std::fs::File;
use std::io::BufReader;

fn main() {
    let config_file = File::open("config").expect("Couldn't open file");
    let buf_reader = BufReader::new(config_file);
    let config: Config = serde_json::from_reader(buf_reader).expect("Couldn't deserialize config");

    let render_config = config.render_config;

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

    let cam = camera::Camera::new(config.camera_config);
    let w = generate_world();

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
