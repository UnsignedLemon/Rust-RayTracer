// Math support of ray tracer.

//---------------------------    Module math_support    ----------------------------------------------------------

use rand::Rng;
use std::ops;
pub const EPS: f64 = 0.000000001;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    // Not using template. It's f32.
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, val: f64) -> Vec3 {
        Vec3 {
            x: self.x * val,
            y: self.y * val,
            z: self.z * val,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, val: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * val.x,
            y: self.y * val.y,
            z: self.z * val.z,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, val: Vec3) -> Vec3 {
        Vec3 {
            x: self * val.x,
            y: self * val.y,
            z: self * val.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, val: f64) -> Vec3 {
        if val == 0.0 {
            dbg!("Divied by zero");
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            Vec3 {
                x: self.x / val,
                y: self.y / val,
                z: self.z / val,
            }
        }
    }
}

//------------------------------    Implementation of Vec3    ---------------------------------
impl Vec3 {
    pub fn make_vec3(x: f64, y: f64, z: f64) -> Vec3 {
        // Construct Function of Vec3
        Vec3 { x, y, z }
    }

    pub fn get_len(self) -> f64 {
        let sq_len: f64 = self.x * self.x + self.y * self.y + self.z * self.z;
        sq_len.sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let len: f64 = self.get_len();
        self / len
    }

    pub fn sqrt_for_gamma_correction(self) -> Vec3 {
        Vec3::make_vec3(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }
}

//-------------------------------    Global utilities    ------------------------------------------
pub fn dot(val1: Vec3, val2: Vec3) -> f64 {
    val1.x * val2.x + val1.y * val2.y + val1.z * val2.z
}

pub fn cross(val1: Vec3, val2: Vec3) -> Vec3 {
    Vec3 {
        x: val1.y * val2.z - val1.z * val2.y,
        y: val1.z * val2.x - val1.x * val2.z,
        z: val1.x * val2.y - val1.y * val2.x,
    }
}

pub fn close_to(val: f64, target: f64) -> bool {
    // Target shall be 0/1.
    (val > target - EPS) && (val < target + EPS)
}

pub fn close_to_unitary(val: Vec3) -> bool {
    close_to(val.get_len(), 1.0)
}

pub fn rand_0_1() -> f64 {
    // Random number between 0 & 1.
    let mut rng = rand::thread_rng();
    let val: f64 = rng.gen::<f64>();
    val
}

pub fn rand_abs_1() -> f64 {
    // Random number between -1 & 1.
    rand_0_1() * 2.0 - 1.0
}

pub fn rand_normalized_vec() -> Vec3 {
    let mut res: Vec3 = Vec3::make_vec3(rand_abs_1(), rand_abs_1(), rand_abs_1());
    while res.get_len() > 1.0 {
        res = Vec3::make_vec3(rand_abs_1(), rand_abs_1(), rand_abs_1());
    }
    res.normalize()
}

pub fn rand_normalized_disk_vec() -> Vec3 {
    let mut res: Vec3 = Vec3::make_vec3(rand_abs_1(), rand_abs_1(), 0.0);
    while res.get_len() > 1.0 {
        res = Vec3::make_vec3(rand_abs_1(), rand_abs_1(), 0.0);
    }
    res.normalize()
}

pub fn is_front_face(dir: Vec3, normal: Vec3) -> bool {
    dot(dir, normal) < EPS
}

pub fn dist(pos1: Vec3, pos2: Vec3) -> f64 {
    (pos2 - pos1).get_len()
}

pub fn reflect(dir: Vec3, normal: Vec3) -> Vec3 {
    // Normal must be normalized.
    dir - (dot(dir, normal) * normal) * 2.0
}

pub fn refract(dir: Vec3, normal: Vec3, ratio: f64) -> Vec3 {
    let cos_theta = -dot(dir, normal);
    let res_perp = ratio * (dir + cos_theta * normal);
    let perp_len = res_perp.get_len();
    let perp_len = (1.0 - perp_len * perp_len).sqrt();
    let res_para = if is_front_face(dir, normal) {
        -1.0
    } else {
        1.0
    } * perp_len
        * normal;

    res_perp + res_para
}

pub fn print_vec3(xx: Vec3) {
    println!("x={}, y={}, z={}", xx.x, xx.y, xx.z);
}
