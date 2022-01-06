use crate::rand;
use crate::Color;
use crate::Point;
use crate::Ray;
use crate::Vec3;
use crate::vec::dot;
use std::borrow::Borrow;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Point,
    pub n: Vec3,
    pub front_face: bool,
}

#[derive(Clone, Copy)]
pub struct WorldHitRecord<'a> {
    pub hit_record: HitRecord,
    pub material: &'a dyn Material,
    pub object_id: u32,
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
    fn scatter(&self, _: Ray, h: HitRecord) -> Option<ScatterResult> {
        let mut new_ray = Ray {
            origin: h.p,
            d: rand::random_in_hemisphere(h.n),
        };

        if new_ray.d.length() < 1e-8 {
            new_ray.d = h.n;
        }

        Some(ScatterResult {
            attenuation: self.albedo,
            scattered_ray: new_ray,
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f32,
}

impl Material for Metal {
    fn scatter(&self, r: Ray, h: HitRecord) -> Option<ScatterResult> {
        let op = h.p - r.origin;
        let pop = (h.p + op) + h.n * dot(op, h.n).abs() * 2.0;
        let new_ray = Ray {
            origin: h.p,
            d: pop - h.p + self.fuzziness * rand::random_in_sphere(),
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
    hits: &[WorldHitRecord<'a>],
    tmin: f32,
    tmax: f32,
) -> Option<WorldHitRecord<'a>> {
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

    pub fn hit(&self, r: Ray) -> Vec<WorldHitRecord> {
        let mut hits = Vec::<WorldHitRecord>::new();
        for (i, object) in self.objects.iter().enumerate() {
            let mut obj_hits = object
                .hit(r)
                .into_iter()
                .map(|h| WorldHitRecord {
                    hit_record: h,
                    material: object.material.borrow(),
                    object_id: i as u32,
                })
                .collect();
            hits.append(&mut obj_hits);
        }
        hits
    }
}
