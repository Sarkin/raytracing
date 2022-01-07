use crate::vec::dot;
use crate::Vec3;
use rand::random;

pub fn get_random_offset() -> f32 {
    random::<f32>() * 2.0 - 1.0
}

pub fn random_in_sphere() -> Vec3 {
    loop {
        let v = Vec3 {
            x: get_random_offset(),
            y: get_random_offset(),
            z: get_random_offset(),
        };
        if v.length() <= 1.0 {
            return v;
        }
    }
}

pub fn random_in_hemisphere(n: Vec3) -> Vec3 {
    let v = random_in_sphere();
    match dot(v, n) < 0.0 {
        true => -v,
        _ => v,
    }
}
