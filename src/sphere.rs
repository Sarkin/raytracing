use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec::dot;
use crate::vec::Point;

use serde::{Deserialize, Serialize};
use typetag;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub o: Point,
    pub r: f32,
}

enum QuadraticSolution {
    NoSolution,
    OneSolution(f32),
    TwoSolutions(f32, f32),
}

// a*x^2 + b*x + c = 0
fn solve_quadratic(a: f32, b: f32, c: f32) -> QuadraticSolution {
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        return QuadraticSolution::NoSolution;
    } else if d == 0.0 {
        return QuadraticSolution::OneSolution(-b / (2.0 * a));
    }

    let sqrt = d.sqrt();
    QuadraticSolution::TwoSolutions((-b - sqrt) / (2.0 * a), (-b + sqrt) / (2.0 * a))
}

enum RaySphereIntersection {
    NoIntersection,
    OnePoint(f32),
    TwoPoints(f32, f32),
}

fn intersect_sphere_ray(s: Sphere, r: Ray) -> RaySphereIntersection {
    let oc = r.origin - s.o;

    let a = dot(r.d, r.d);
    let b = 2.0 * dot(r.d, oc);
    let c = dot(oc, oc) - s.r * s.r;

    match solve_quadratic(a, b, c) {
        QuadraticSolution::NoSolution => RaySphereIntersection::NoIntersection,
        QuadraticSolution::OneSolution(t) => RaySphereIntersection::OnePoint(t),
        QuadraticSolution::TwoSolutions(t1, t2) => RaySphereIntersection::TwoPoints(t1, t2),
    }
}

#[typetag::serde]
impl Hittable for Sphere {
    fn hit(&self, r: Ray) -> Vec<HitRecord> {
        let mut hits: Vec<HitRecord> = Vec::new();

        let get_hit_record = |t: f32| {
            let p = r.at(t);
            let outward_n = (p - self.o) / self.r;
            let front_face = dot(outward_n, r.origin - p) >= 0.0;
            let n = if front_face { outward_n } else { -outward_n };
            HitRecord {
                t,
                p,
                n,
                front_face,
            }
        };

        match intersect_sphere_ray(*self, r) {
            RaySphereIntersection::OnePoint(t) => hits.push(get_hit_record(t)),
            RaySphereIntersection::TwoPoints(t1, t2) => {
                hits.push(get_hit_record(t1));
                hits.push(get_hit_record(t2));
            }
            _ => {}
        }

        hits
    }
}
