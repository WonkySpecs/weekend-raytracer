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

    fn vec_add(&self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]]
        }
    }

    fn scalar_add(&self, other: f64) -> Vec3 {
        Vec3 {
            v: [self.v[0] + other,
                self.v[1] + other,
                self.v[2] + other]
        }
    }

    fn vec_sub(&self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] - other.v[0],
                self.v[1] - other.v[1],
                self.v[2] - other.v[2]]
        }
    }

    fn scalar_sub(&self, other: f64) -> Vec3 {
        Vec3 {
            v: [self.v[0] - other,
                self.v[1] - other,
                self.v[2] - other]
        }
    }

    fn inverse(&self) -> Vec3 {
        Vec3 {
            v: [-self.v[0],
                -self.v[1],
                -self.v[2]]
        }
    }

    fn vec_mult(&self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] * other.v[0],
                self.v[1] * other.v[1],
                self.v[2] * other.v[2]]
        }
    }

    fn scalar_mult(&self, t: f64) -> Vec3 {
        Vec3 {
            v: [self.v[0] * t,
                self.v[1] * t,
                self.v[2] * t]
        }
    }

    fn vec_div(&self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] / other.v[0],
                self.v[1] / other.v[1],
                self.v[2] / other.v[2]]
        }
    }

    fn scalar_div(&self, t: f64) -> Vec3 {
        Vec3 {
            v: [self.v[0] / t,
                self.v[1] / t,
                self.v[2] / t]
        }
    }

    fn dot(&self, other: Vec3) -> f64 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[1] * other.v[2] - self.v[2] * other.v[1],
                -(self.v[0] * other.v[2] - self.v[2] * other.v[0]),
                self.v[0] * other.v[1] - self.v[1] * other.v[2]]
        }
    }

    fn len(&self) -> f64 {
        (self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]).sqrt()
    }

    fn len_squared(&self) -> f64 {
        self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]
    }

    fn unit_vec(&self) -> Vec3 {
        Vec3 {
            v: [self.v[0] / self.len(),
                self.v[1] / self.len(),
                self.v[2] / self.len()]
        }
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