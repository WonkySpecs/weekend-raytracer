#[derive(Copy, Clone)]
pub struct Vec3 {
    pub v: [f64; 3]
}

impl Vec3 {
    pub fn r(&self) -> f64 {
        self.v[0]
    }

    pub fn g(&self) -> f64 {
        self.v[1]
    }

    pub fn b(&self) -> f64 {
        self.v[2]
    }

    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }

    pub fn z(&self) -> f64 {
        self.v[2]
    }

    pub fn convert_to_ints(&self) -> [i32; 3] {
        let v: Vec<i32> = self.v.iter()
            .map(|c| colour_range_map(c))
            .collect();
        assert_eq!(v.len(), 3);
        [v[0], v[1], v[2]]
    }

    pub fn vec_add(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]]
        }
    }

    pub fn scalar_add(&self, other: f64) -> Vec3 {
        Vec3 {
            v: [self.v[0] + other,
                self.v[1] + other,
                self.v[2] + other]
        }
    }

    pub fn vec_sub(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] - other.v[0],
                self.v[1] - other.v[1],
                self.v[2] - other.v[2]]
        }
    }

    pub fn scalar_sub(&self, other: f64) -> Vec3 {
        Vec3 {
            v: [self.v[0] - other,
                self.v[1] - other,
                self.v[2] - other]
        }
    }

    pub fn inverse(&self) -> Vec3 {
        Vec3 {
            v: [-self.v[0],
                -self.v[1],
                -self.v[2]]
        }
    }

    pub fn vec_mult(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] * other.v[0],
                self.v[1] * other.v[1],
                self.v[2] * other.v[2]]
        }
    }

    pub fn scalar_mult(&self, t: f64) -> Vec3 {
        Vec3 {
            v: [self.v[0] * t,
                self.v[1] * t,
                self.v[2] * t]
        }
    }

    pub fn vec_div(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] / other.v[0],
                self.v[1] / other.v[1],
                self.v[2] / other.v[2]]
        }
    }

    pub fn scalar_div(&self, t: f64) -> Vec3 {
        Vec3 {
            v: [self.v[0] / t,
                self.v[1] / t,
                self.v[2] / t]
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[1] * other.v[2] - self.v[2] * other.v[1],
                -(self.v[0] * other.v[2] - self.v[2] * other.v[0]),
                self.v[0] * other.v[1] - self.v[1] * other.v[2]]
        }
    }

    pub fn len(&self) -> f64 {
        (self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]).sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]
    }

    pub fn unit_vec(&self) -> Vec3 {
        Vec3 {
            v: [self.v[0] / self.len(),
                self.v[1] / self.len(),
                self.v[2] / self.len()]
        }
    }
}

fn colour_range_map(c: &f64) -> i32 {
    (c * 255.99) as i32
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn at_param(&self, t: f64) -> Vec3 {
        self.origin.vec_add(&self.direction.scalar_mult(t))
    }
}

pub struct Hit {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3
}

pub trait Hitable {
    fn get_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct Sphere {
    pub center: Vec3,
    pub r: f64
}

impl Hitable for Sphere {
    fn get_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let to_center = ray.origin.vec_sub(&self.center);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * to_center.dot(&ray.direction);
        let c = to_center.dot(&to_center) - self.r * self.r;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let root = (-b - discriminant.sqrt()) / (2.0 * a);
            if root > t_min && root < t_max {
                let hit_point = ray.at_param(root);
                return Option::Some(
                    Hit {
                        t: root,
                        p: hit_point,
                        normal: hit_point.vec_sub(&self.center).scalar_div(self.r)
                    }
                )
            }
            let root = (-b + discriminant.sqrt()) / (2.0 * a);
            if root > t_min && root < t_max {
                let hit_point = ray.at_param(root);
                return Option::Some(
                    Hit {
                        t: root,
                        p: hit_point,
                        normal: hit_point.vec_sub(&self.center).scalar_div(self.r)
                    }
                )
            }
        }
        Option::None
    }
}

pub struct World {
    pub objects: Vec<Box<dyn Hitable>>
}