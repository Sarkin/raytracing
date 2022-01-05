use crate::ray::Ray;
use crate::vec::Point;
use crate::vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Point,
    pub n: Vec3,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: Ray) -> Vec<HitRecord>;
}

pub fn get_closest_hit_in_range(hits: &[HitRecord], tmin: f32, tmax: f32) -> Option<HitRecord> {
    hits.iter()
        .filter(|h| tmin <= h.t && h.t <= tmax)
        .reduce(|a, b| if a.t <= b.t { a } else { b })
        .copied()
}

#[derive(Default)]
pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn add_object(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for World {
    fn hit(&self, r: Ray) -> Vec<HitRecord> {
        let mut hits = Vec::<HitRecord>::new();
        for object in &self.objects {
            let mut obj_hits = object.hit(r);
            hits.append(&mut obj_hits);
        }
        hits
    }
}
