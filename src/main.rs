use std::fs;

struct Vec3 {
    v: [f64; 3]
}

impl Vec3 {
    fn r(&self) -> f64 {
        self.v[0]
    }

    fn g(&self) -> f64 {
        self.v[1]
    }

    fn b(&self) -> f64 {
        self.v[2]
    }

    fn x(&self) -> f64 {
        self.v[0]
    }

    fn y(&self) -> f64 {
        self.v[1]
    }

    fn z(&self) -> f64 {
        self.v[2]
    }

    fn convert_to_ints(&self) -> [i32; 3] {
        let v: Vec<i32> = self.v.iter()
            .map(|c| colour_range_map(c))
            .collect();
        assert_eq!(v.len(), 3);
        [v[0], v[1], v[2]]
    }
}

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

fn colour_range_map(c: &f64) -> i32 {
    (c * 255.99) as i32
}