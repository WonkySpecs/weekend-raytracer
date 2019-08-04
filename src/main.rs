use std::fs;

fn main() {
    let width = 200;
    let height = 100;
    let mut content = format!("P3\n{} {}\n255\n", width, height);
    for j in (0..height).rev() {
        for i in 0..width {
            let r = i as f64 / width as f64;
            let g = j as f64 / height as f64;
            let b = 0.2;
            content += &format!("{} {} {} ",
                             colour_range_map(r),
                             colour_range_map(g),
                             colour_range_map(b));
        }
        content += "\n";
    }
    fs::write("out.ppm", content);
}

fn colour_range_map(c: f64) -> i32 {
    (c * 255.99) as i32
}