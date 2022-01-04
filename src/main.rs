use std::io;

fn ppm_print(img: &[Vec<(i32, i32, i32)>]) {
    println!("P3\n{} {}\n255", img.len(), img[0].len());
    for row in img {
        for (r, g, b) in row {
            println!("{} {} {}", r, g, b);
        }
    }
}

fn main() {
    let img_height: usize = 1024;
    let img_width: usize = 1024;

    let mut img = vec![vec![(0, 0, 0); img_width]; img_height];

    for (i, row) in img.iter_mut().enumerate() {
        eprintln!("Rows remaining {}", img_height - i);
        for (j, cell) in row.iter_mut().enumerate() {
            let (r, g, b) = ((j as f32 / (img_height - 1) as f32) * 255.999, ((img_height - i - 1) as f32 / img_width as f32) * 255.999, (0.25 * 255.999) as f32);
            *cell = (r as i32, g as i32, b as i32);
        }
    }

    eprintln!("Printing image..");
    ppm_print(&img);
    eprintln!("Done!");
}
