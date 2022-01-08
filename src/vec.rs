use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - v.y * u.z,
        y: -(u.x * v.z - v.x * u.z),
        z: u.x * v.y - v.x * u.y,
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, m: f32) -> Self {
        Self {
            x: self.x * m,
            y: self.y * m,
            z: self.z * m,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, m: f32) -> Self {
        self * (1.0 / m)
    }
}

impl Vec3 {
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
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

#[cfg(test)]
mod tests {
    use super::cross;
    use super::Vec3;

    #[test]
    fn test_cross() {
        let u = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(
            cross(u, v),
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
        );
    }
}
