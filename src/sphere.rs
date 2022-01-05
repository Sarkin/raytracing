use crate::vec::dot;
use crate::Ray;
use crate::Point;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub o: Point,
    pub r: f32
}

enum QuadraticSolution {
    NoSolution,
    OneSolution(f32),
    TwoSolutions(f32, f32)
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
    OnePoint(Point),
    TwoPoints(Point, Point)
}

fn intersect_sphere_ray(s: Sphere, r: Ray) -> RaySphereIntersection {
    let oc = r.origin - s.o;

    let a = dot(r.d, r.d);
    let b = 2.0 * dot(r.d, oc);
    let c = dot(oc, oc) - s.r * s.r;

    match solve_quadratic(a, b, c) {
        QuadraticSolution::NoSolution => RaySphereIntersection::NoIntersection,
        QuadraticSolution::OneSolution(t) => RaySphereIntersection::OnePoint(r.at(t)),
        QuadraticSolution::TwoSolutions(t1, t2) => RaySphereIntersection::TwoPoints(r.at(t1), r.at(t2))
    }
}

pub fn get_closest_sphere_ray_intersection(s: Sphere, r: Ray) -> Option<Point> {
    match intersect_sphere_ray(s, r) {
        RaySphereIntersection::NoIntersection => None,
        RaySphereIntersection::OnePoint(p) => Some(p),
        RaySphereIntersection::TwoPoints(p, _) => Some(p) // TODO: p is not closer if t is negative
    }
}
