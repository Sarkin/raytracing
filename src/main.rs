mod ray;
mod vec;
mod sphere;
mod hittable;

use ray::Ray;
use sphere::Sphere;
use vec::Color;
use vec::Point;
use vec::Vec3;
use hittable::Hittable;
use hittable::get_closest_hit_in_range;
use hittable::World;

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
    Color { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Color { x: 0.5, y: 0.7, z: 1.0 } * t
}

fn get_world() -> World {
    let s = Sphere { o: Point{ x: 0.0, y: 0.0, z: -1.0 }, r: 0.5 };
    let mut w: World = Default::default();
    w.add_object(Box::new(s));
    w
}

fn ray_color(r: Ray) -> Color {
    let w = get_world();
    match get_closest_hit_in_range(&w.hit(r), 0.0, 100.0) {
        None => ray_color_blue_gradient(r),
        Some(h) => {
            (h.n + Vec3 { x: 1.0, y: 1.0, z: 1.0 }) * 0.5 // TODO: consider op Vec + f32?
        }
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width: usize = 1024;
    let img_height: usize = (img_width as f32 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin: Point = Default::default();
    let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical = Vec3 { x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3 { x: 0.0, y: 0.0, z: focal_length };

    let mut img = vec![vec![Color { x: 0.0, y: 0.0, z: 0.0 }; img_width]; img_height];

    for (i, row) in img.iter_mut().enumerate() {
        eprintln!("Rows remaining {}", img_height - i);
        for (j, cell) in row.iter_mut().enumerate() {
            let pi = (img_height - i - 1) as f32 / (img_height - 1) as f32;
            let pj = j as f32 / (img_width - 1) as f32;
            let r = Ray { origin, d: lower_left_corner + vertical * pi + horizontal * pj };
            *cell = ray_color(r);
        }
    }

    eprintln!("Printing image..");
    ppm_print(&img);
    eprintln!("Done!");
}
