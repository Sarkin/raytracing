use std::io;
use std::ops;
use std::fmt;

#[derive(Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    
    fn mul(self, m: f32) -> Self {
        Self {x: self.x * m, y: self.y * m, z: self.z * m}
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m = 255.999;
        let vn = *self * m;
        write!(f, "{} {} {}", vn.x as i32, vn.y as i32, vn.z as i32)
    }
}

use Vec3 as Point;
use Vec3 as Color;

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

    let mut img = vec![vec![Color {x: 0.0, y: 0.0, z: 0.0}; img_width]; img_height];

    for (i, row) in img.iter_mut().enumerate() {
        eprintln!("Rows remaining {}", img_height - i);
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = Color {x: j as f32 / (img_height - 1) as f32, y: (img_height - i - 1) as f32 / img_width as f32, z: 0.25 as f32};
        }
    }

    eprintln!("Printing image..");
    ppm_print(&img);
    eprintln!("Done!");
}
