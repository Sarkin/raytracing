use std::ops;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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

pub use Vec3 as Point;
pub use Vec3 as Color;
