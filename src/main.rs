mod ray;
mod vec;

use ray::Ray;
use vec::Color;
use vec::Point;
use vec::Vec3;

fn ppm_print(img: &[Vec<Color>]) {
    println!("P3\n{} {}\n255", img.len(), img[0].len());
    for row in img {
        for pixel_color in row {
            println!("{}", pixel_color);
        }
    }
}

fn main() {
    let img_height: usize = 1024;
    let img_width: usize = 1024;

    let mut img = vec![vec![Color { x: 0.0, y: 0.0, z: 0.0 }; img_width]; img_height];

    for (i, row) in img.iter_mut().enumerate() {
        eprintln!("Rows remaining {}", img_height - i);
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = Color { x: j as f32 / (img_height - 1) as f32, y: (img_height - i - 1) as f32 / img_width as f32, z: 0.25 as f32 };
        }
    }

    eprintln!("Printing image..");
    ppm_print(&img);
    eprintln!("Done!");

    // let r = Ray { origin: Point { x: 0.0, y: 0.0, z: 0.0 }, d: Vec3 { x: 1.0, y: 2.0, z: 3.0 } };
    // eprintln!("{:?}", r.at(1.5));
}
