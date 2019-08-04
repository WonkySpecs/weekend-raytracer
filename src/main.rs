use std::fs;
use data_structures::Vec3;

mod data_structures;

fn main() {
    let width = 200;
    let height = 100;
    let mut content = format!("P3\n{} {}\n255\n", width, height);
    for j in (0..height).rev() {
        for i in 0..width {
            let c = Vec3 {
                v: [i as f64 / width as f64,
                    j as f64 / height as f64,
                    0.4]
            };
            let c = c.convert_to_ints();
            content += &format!("{} {} {} ",
                                c[0],
                                c[1],
                                c[2])
        }
        content += "\n";
    }
    fs::write("out.ppm", content);
}