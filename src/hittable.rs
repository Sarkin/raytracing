use crate::rand;
use crate::vec::dot;
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
pub struct WorldHitRecord<'a> {
    pub hit_record: HitRecord,
    pub material: &'a (dyn Material + Sync),
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
            origin: h.p + 1e-3 * h.n,
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

fn reflect_vector(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

impl Material for Metal {
    fn scatter(&self, r: Ray, h: HitRecord) -> Option<ScatterResult> {
        let reflected_d = reflect_vector(r.d, h.n);
        let new_ray = Ray {
            origin: h.p + 1e-3 * h.n,
            d: reflected_d + self.fuzziness * rand::random_in_sphere(),
        };

        Some(ScatterResult {
            attenuation: self.albedo,
            scattered_ray: new_ray,
        })
    }
}

pub struct Dielectric {
    pub ir: f32,
}

impl Material for Dielectric {
    fn scatter(&self, r: Ray, h: HitRecord) -> Option<ScatterResult> {
        let unit_d = r.d.unit();
        let refraction_rate = if h.front_face { 1.0 / self.ir } else { self.ir };
        let cos_theta = dot(-unit_d, h.n).min(1.0);
        let in_x = unit_d + cos_theta * h.n;
        let sin_theta = in_x.length();

        let d: Vec3;
        let origin: Point;
        if (sin_theta * refraction_rate).abs() > 1.0 {
            d = reflect_vector(r.d, h.n);
            origin = h.p + h.n * 1e-3;
        } else {
            let ray_x = refraction_rate * in_x;
            let ray_y = -(1.0 - ray_x.length_squared()).sqrt() * h.n;
            d = ray_x + ray_y;
            origin = h.p - h.n * 1e-3;
        }

        Some(ScatterResult {
            attenuation: Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            scattered_ray: Ray { origin, d },
        })
    }
}

pub struct Object {
    pub material: Box<dyn Material + Sync>,
    pub hittable: Box<dyn Hittable + Sync>,
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
