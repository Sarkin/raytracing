mod ray;
mod vec;

use ray::Ray;
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

fn ray_color(r: Ray) -> Color {
    let u = r.d.unit();
    let t = (u.y + 1.0) / 2.0;
    Color { x: 1.0, y: 1.0, z: 1.0 } * t + Color { x: 0.5, y: 0.7, z: 1.0 } * (1.0 - t)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width: usize = 400;
    let img_height: usize = (img_width as f32 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin: Point = Default::default();
    let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical = Vec3 { x: 0.0, y: viewport_height, z: 0.0 };
    eprintln!("Printing image..");
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3 { x: 0.0, y: 0.0, z: focal_length };
    eprintln!("Printing image..");

    let mut img = vec![vec![Color { x: 0.0, y: 0.0, z: 0.0 }; img_width]; img_height];

    for (i, row) in img.iter_mut().enumerate() {
        eprintln!("Rows remaining {}", img_height - i);
        for (j, cell) in row.iter_mut().enumerate() {
            let pi = i as f32 / (img_height - 1) as f32;
            let pj = j as f32 / (img_width - 1) as f32;
            let r = Ray { origin: origin, d: lower_left_corner + vertical * pi + horizontal * pj };
            *cell = ray_color(r);
        }
    }

    eprintln!("Printing image..");
    ppm_print(&img);
    eprintln!("Done!");
}
