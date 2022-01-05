use crate::vec;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: vec::Point,
    pub d: vec::Vec3
}

impl Ray {
    pub fn at(&self, t: f32) -> vec::Point {
        self.origin + self.d * t
    }
}
