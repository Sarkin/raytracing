use crate::Point;
use crate::Ray;
use crate::Vec3;

pub struct Camera {
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let origin: Point = Default::default();
        let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
        let vertical = Vec3 { x: 0.0, y: viewport_height, z: 0.0 };
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3 { x: 0.0, y: 0.0, z: focal_length };

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray { origin: self.origin, d: self.lower_left_corner + self.vertical * u + self.horizontal * v }
    }
}
