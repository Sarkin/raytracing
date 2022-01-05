mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec;

use hittable::get_closest_hit_in_range;
use hittable::Hittable;
use hittable::World;
use rand::random;
use ray::Ray;
use sphere::Sphere;
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
    w.add_object(Box::new(Sphere {
        o: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        r: 0.5,
    }));
    w.add_object(Box::new(Sphere {
        o: Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        r: 100.0,
    }));
    w
}

fn ray_color(r: Ray, w: &World) -> Color {
    match get_closest_hit_in_range(&w.hit(r), 0.0, 100.0) {
        None => ray_color_blue_gradient(r),
        Some(h) => {
            (h.n + Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }) * 0.5 // TODO: consider op Vec + f32?
        }
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width: usize = 1024;
    let img_height: usize = (img_width as f32 / aspect_ratio) as usize;
    let number_of_samples = 10;

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

    for (i, row) in img.iter_mut().enumerate() {
        eprintln!("Rows remaining {}", img_height - i);
        for (j, cell) in row.iter_mut().enumerate() {
            let (r, c) = ((img_height - i - 1) as f32, j as f32);

            let get_random_offset = || random::<f32>() * 2.0 - 1.0;
            for _ in 0..number_of_samples {
                let u = (r + get_random_offset()) / (img_height - 1) as f32;
                let v = (c + get_random_offset()) / (img_width - 1) as f32;
                *cell = *cell + ray_color(cam.get_ray(u, v), &w);
            }

            *cell = *cell / (number_of_samples as f32);
        }
    }

    eprintln!("Printing image..");
    ppm_print(&img);
    eprintln!("Done!");
}
