mod camera;
mod hittable;
mod rand;
mod ray;
mod sphere;
mod vec;

use hittable::get_closest_hit_in_range;
use hittable::Dielectric;
use hittable::Lambertian;
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
            r: 0.4,
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
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            r: 0.4,
        }),
        material: Box::new(Dielectric { ir: 1.5 }),
    });
    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: Point {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            r: 0.4,
        }),
        material: Box::new(Metal {
            albedo: Color {
                x: 0.6,
                y: 0.5,
                z: 0.5,
            },
            fuzziness: 0.0,
        }),
    });
    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: Point {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            r: 100.0,
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

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width: usize = 1200;
    let img_height: usize = (img_width as f32 / aspect_ratio) as usize;
    let number_of_samples = 200;
    let depth = 10;

    let cam = camera::Camera::new();

    let mut img = vec![
        vec![
            Color {
                x: 0.0,
                y: 0.0,
                z: 0.0
            };
            img_width
        ];
        img_height
    ];

    let w = get_world();

    let c_rows = AtomicUsize::new(0);
    img.par_iter_mut().enumerate().for_each(|(i, row)| {
        for (j, cell) in row.iter_mut().enumerate() {
            let (r, c) = ((img_height - i - 1) as f32, j as f32);

            for _ in 0..number_of_samples {
                let u = (r + rand::get_random_offset()) / (img_height - 1) as f32;
                let v = (c + rand::get_random_offset()) / (img_width - 1) as f32;
                *cell = *cell + ray_color(cam.get_ray(u, v), &w, depth);
            }

            *cell = *cell / (number_of_samples as f32);
        }
        let rows_done = c_rows.fetch_add(1, Ordering::SeqCst);
        eprintln!("Rows remaining {}", img_height - rows_done);
    });

    eprintln!("Printing image..");
    ppm_print(&img);
    eprintln!("Done!");
}
