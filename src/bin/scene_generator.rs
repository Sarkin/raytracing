use raytracing::camera;
use raytracing::rand;
use raytracing::scene::Scene;

use raytracing::hittable;
use raytracing::sphere::Sphere;
use raytracing::vec::Color;
use raytracing::vec::Point;
use raytracing::vec::Vec3;

use hittable::Dielectric;
use hittable::Lambertian;
use hittable::Material;
use hittable::Metal;
use hittable::Object;
use hittable::World;

use std::env;

fn get_scene() -> Scene {
    let mut w: World = Default::default();
    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: Point {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            r: 0.5,
        }),
        material: Box::new(Lambertian {
            albedo: Color {
                x: 0.1,
                y: 0.2,
                z: 0.3,
            },
        }),
    });
    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: Point {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            r: 0.5,
        }),
        material: Box::new(Dielectric { ir: 1.5 }),
    });
    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: Point {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            r: 0.5,
        }),
        material: Box::new(Metal {
            albedo: Color {
                x: 0.8,
                y: 0.8,
                z: 0.8,
            },
            fuzziness: 0.0,
        }),
    });
    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: Point {
                x: 0.0,
                y: -500.5,
                z: -1.0,
            },
            r: 500.0,
        }),
        material: Box::new(Lambertian {
            albedo: Color {
                x: 0.1,
                y: 0.7,
                z: 0.3,
            },
        }),
    });

    let lookfrom = Point {
        x: 1.0,
        y: 0.0,
        z: 1.0,
    };
    let lookat = Point {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    Scene {
        camera_config: camera::CameraConfig {
            lookfrom,
            lookat,
            vup,
            vfov: 90.0,
            aspect_ratio: 16.0 / 9.0,
            aperture: 0.1,
            focus_dist: 2.0,
        },
        world: w,
    }
}

fn generate_scene() -> Scene {
    let mut w: World = Default::default();

    let earth_radius = 500.0;
    let earth_center = Point {
        x: 0.0,
        y: -earth_radius,
        z: 0.0,
    };

    w.add_object(Object {
        hittable: Box::new(Sphere {
            o: earth_center,
            r: earth_radius,
        }),
        material: Box::new(Lambertian {
            albedo: Color {
                x: 0.8,
                y: 0.8,
                z: 0.8,
            },
        }),
    });

    for xi in -10..10 {
        for zi in -10..10 {
            let (x, z) = (
                (xi * 3) as f32 + rand::get_random_offset(),
                (zi * 3) as f32 + rand::get_random_offset(),
            );
            let r = 0.5;
            let c =
                (Point { x, y: 0.0, z } - earth_center).unit() * (earth_radius + r) + earth_center;

            let material_p = rand::get_random_float();
            let material: Box<dyn Material>;
            if material_p > 0.8 {
                material = Box::new(Metal {
                    albedo: Color {
                        x: rand::get_random_float(),
                        y: rand::get_random_float(),
                        z: rand::get_random_float(),
                    },
                    fuzziness: 0.1 * rand::get_random_float(),
                });
            } else if material_p > 0.7 {
                material = Box::new(Dielectric { ir: 1.5 });
            } else {
                material = Box::new(Lambertian {
                    albedo: Color {
                        x: rand::get_random_float(),
                        y: rand::get_random_float(),
                        z: rand::get_random_float(),
                    },
                });
            }
            w.add_object(Object {
                hittable: Box::new(Sphere { o: c, r }),
                material,
            });
        }
    }

    let lookfrom = Point {
        x: 13.0,
        y: 5.0,
        z: 2.0,
    };
    let lookat = Point {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    Scene {
        camera_config: camera::CameraConfig {
            lookfrom,
            lookat,
            vup,
            vfov: 90.0,
            aspect_ratio: 16.0 / 9.0,
            aperture: 0.1,
            focus_dist: 12.0,
        },
        world: w,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene = match args[1].as_str() {
        "static" => get_scene(),
        _ => generate_scene(),
    };
    println!("{}", serde_json::to_string_pretty(&scene).unwrap());
}
