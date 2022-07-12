//----------------------------------    Struct Ray    ------------------------------------
use crate::math_support;

pub use math_support::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pos: Vec3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn make_ray(pos: Vec3, dir: Vec3, tm: f64) -> Ray {
        Ray {
            pos,
            dir: dir.normalize(),
            tm,
        }
    }
    pub fn get_pos(&self) -> Vec3 {
        self.pos
    }
    pub fn get_dir(&self) -> Vec3 {
        self.dir
    }
    pub fn get_tm(&self) -> f64 {
        self.tm
    }
}
