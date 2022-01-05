use crate::rand;
use crate::Color;
use crate::Point;
use crate::Ray;
use crate::Vec3;
use std::borrow::Borrow;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Point,
    pub n: Vec3,
    pub front_face: bool,
}

#[derive(Clone, Copy)]
pub struct HitRecordWithMaterial<'a> {
    pub hit_record: HitRecord,
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, r: Ray) -> Vec<HitRecord>;
}

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered_ray: Ray,
}

pub trait Material {
    fn scatter(&self, r: Ray, h: HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r: Ray, h: HitRecord) -> Option<ScatterResult> {
        let new_ray = Ray {
            origin: h.p,
            d: h.n + rand::random_in_sphere(),
        };
        Some(ScatterResult {
            attenuation: self.albedo,
            scattered_ray: new_ray,
        })
    }
}

pub struct Object {
    pub material: Box<dyn Material>,
    pub hittable: Box<dyn Hittable>,
}

impl Hittable for Object {
    fn hit(&self, r: Ray) -> Vec<HitRecord> {
        self.hittable.hit(r)
    }
}

pub fn get_closest_hit_in_range<'a>(
    hits: &[HitRecordWithMaterial<'a>],
    tmin: f32,
    tmax: f32,
) -> Option<HitRecordWithMaterial<'a>> {
    hits.iter()
        .filter(|h| tmin <= h.hit_record.t && h.hit_record.t <= tmax)
        .reduce(|a, b| {
            if a.hit_record.t <= b.hit_record.t {
                a
            } else {
                b
            }
        })
        .copied()
}

#[derive(Default)]
pub struct World {
    objects: Vec<Object>,
}

impl World {
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: Ray) -> Vec<HitRecordWithMaterial> {
        let mut hits = Vec::<HitRecordWithMaterial>::new();
        for object in &self.objects {
            let mut obj_hits = object
                .hit(r)
                .into_iter()
                .map(|h| HitRecordWithMaterial {
                    hit_record: h,
                    material: object.material.borrow(),
                })
                .collect();
            hits.append(&mut obj_hits);
        }
        hits
    }
}
