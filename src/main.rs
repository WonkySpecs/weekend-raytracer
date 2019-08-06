extern crate rand;

use std::fs;
use rand::Rng;

use data_structures::{Vec3, Ray, Sphere, Hitable, Hit, World};

mod data_structures;

const T_MIN: f64 = 0.1;
const T_MAX: f64 = 10.0;
const RAYS_PER_PIXEL: i32 = 10;

fn main() {
    let width = 200;
    let height = 100;
    let mut content = format!("P3\n{} {}\n255\n", width, height);
    let origin = Vec3 { v: [0.0, 0.0, 0.0] };
    let lower_left = Vec3 { v: [-2.0, -1.0, -1.0] };
    let horizontal = Vec3 { v: [4.0, 0.0, 0.0] };
    let vertical = Vec3 { v: [0.0, 2.0, 0.0] };
    let world = setup_world();
    let mut rng = rand::thread_rng();
    for j in (0..height).rev() {
        for i in 0..width {
            let mut tot_color = Vec3 { v: [0.0, 0.0, 0.0] };
            for _ in 0..RAYS_PER_PIXEL {
                let u: f64 = (i as f64 + rng.gen::<f64>()) / width as f64;
                let v: f64 = (j as f64 + rng.gen::<f64>()) / height as f64;
                let r = Ray {
                    origin,
                    direction: lower_left
                        .vec_add(&vertical.scalar_mult(v))
                        .vec_add(&horizontal.scalar_mult(u)),
                };
                let c = color(&r, &world);
                tot_color = tot_color.vec_add(&c);
            }

            let c = tot_color.scalar_div(RAYS_PER_PIXEL as f64).convert_to_ints();
            content += &format!("{} {} {} ",
                                c[0],
                                c[1],
                                c[2])
        }
        content += "\n";
    }
    fs::write("out.ppm", content);
}

fn color(ray: &Ray, world: &World) -> Vec3 {
    if let Some(hit) = closest_collision(ray, world) {
        return hit.normal.scalar_add(1.0).scalar_mult(0.5);
    }
    let unit_direction = ray.direction.unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3 { v: [1.0, 1.0, 1.0] }.scalar_mult(1.0 - t)
        .vec_add(&Vec3 { v: [0.3, 0.5, 1.0] }.scalar_mult(t))
}

fn closest_collision(ray: &Ray, world: &World) -> Option<Hit> {
    world.objects.iter()
        .map(|o| o.get_hit(&ray, T_MIN, T_MAX))
        .filter(|h| h.is_some())
        .map(|h| h.unwrap())
        .max_by(|h1, h2| h2.t.partial_cmp(&h1.t).unwrap())
}

fn setup_world() -> World {
    let head = Sphere {
        center: Vec3 { v: [0.0, 0.0, -1.0] },
        r: 0.6,
    };
    let left_ear = Sphere {
        center: Vec3 { v: [-0.375, 0.375, -1.1] },
        r: 0.35,
    };
    let right_ear = Sphere {
        center: Vec3 { v: [0.375, 0.375, -1.1] },
        r: 0.35,
    };
    let left_eye = Sphere {
        center: Vec3 { v: [-0.12, 0.12, -0.5] },
        r: 0.13,
    };
    let right_eye = Sphere {
        center: Vec3 { v: [0.12, 0.12, -0.5] },
        r: 0.13,
    };
    let nose = Sphere {
        center: Vec3 { v: [0.0, -0.04, -0.48] },
        r: 0.1,
    };
    World {
        objects: vec!(Box::new(head),
                      Box::new(left_ear),
                      Box::new(right_ear),
                      Box::new(left_eye),
                      Box::new(right_eye),
                      Box::new(nose))
    }
}