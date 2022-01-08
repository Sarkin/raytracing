use crate::vec::cross;
use crate::Point;
use crate::Ray;
use crate::Vec3;
use std::f32::consts::PI;

pub struct Camera {
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point,
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Self {
        let theta = vfov / 180.0 * PI;

        let w = (lookfrom - lookat).unit();
        let u = cross(vup, w).unit();
        let v = cross(w, u).unit();

        let h = theta.tan();
        let viewport_height = h;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let origin: Point = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focal_length;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            d: self.lower_left_corner + self.vertical * u + self.horizontal * v - self.origin,
        }
    }
}
