use crate::rand;
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
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov / 180.0 * PI;

        let w = (lookfrom - lookat).unit();
        let u = cross(vup, w).unit();
        let v = cross(w, u).unit();

        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        let origin: Point = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            u,
            v,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * rand::random_in_sphere();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            d: self.lower_left_corner + self.vertical * u + self.horizontal * v
                - self.origin
                - offset,
        }
    }
}
