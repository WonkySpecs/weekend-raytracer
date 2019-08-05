use std::fs;
use data_structures::{Vec3, Ray, Sphere, Hitable};

mod data_structures;

fn main() {
    let width = 200;
    let height = 100;
    let mut content = format!("P3\n{} {}\n255\n", width, height);
    let origin = Vec3 { v: [0.0, 0.0, 0.0] };
    let lower_left = Vec3 { v: [-2.0, -1.0, -1.0] };
    let horizontal = Vec3 { v: [4.0, 0.0, 0.0] };
    let vertical = Vec3 { v: [0.0, 2.0, 0.0] };
    let sphere = Sphere {
        center: Vec3 { v: [0.0, 0.0, -1.0]},
        r: 0.5
    };
    for j in (0..height).rev() {
        for i in 0..width {
            let u: f64 = i as f64 / width as f64;
            let v: f64 = j as f64 / height as f64;
            let r = Ray {
                origin,
                direction: lower_left
                    .vec_add(&vertical.scalar_mult(v))
                    .vec_add(&horizontal.scalar_mult(u)),
            };
            let c = color(&r, &sphere).convert_to_ints();
            content += &format!("{} {} {} ",
                                c[0],
                                c[1],
                                c[2])
        }
        content += "\n";
    }
    fs::write("out.ppm", content);
}

fn color(ray: &Ray, sphere: &Sphere) -> Vec3 {
    if let Some(hit) = sphere.get_hit(&ray, 0.1, 10.0) {
        return hit.normal.scalar_add(1.0).scalar_mult(0.5)
    }
    let unit_direction = ray.direction.unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3 { v: [1.0, 1.0, 1.0] }.scalar_mult(1.0 - t)
        .vec_add(&Vec3 { v: [0.5, 0.7, 1.0] }.scalar_mult(t))
}
