// Mod Material
#![allow(unused_variables)]

use crate::graphics::ray::Ray;
use crate::math_support::*;

//--------------------------    Trait Scatter    ----------------------------------------
pub trait Scatter {
    fn do_scatter(target_ray: &Ray, normal: Vec3) -> Ray;
    // Only do ray scatter, no color mixing.
    // Remember that the pos of target ray must be the hit point.
}

//---------------------------    Struct Lambertian    ------------------------------------

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn make_lmb(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn do_scatter(target_ray: &Ray, normal: Vec3) -> Ray {
        let normal = normal.normalize();
        let new_dir: Vec3 = normal + rand_normalized_vec();
        if close_to(new_dir.get_len(), 0.0) {
            let new_dir = normal;
        } else {
            let new_dir = new_dir.normalize();
        }
        Ray::make_ray(target_ray.get_pos(), new_dir)
    }
}

//---------------------------    Struct Metal    -----------------------------------------

//---------------------------    Enum for Materials    -----------------------------------
pub enum Mat {
    Lmb(Lambertian),
}

impl Mat {
    pub fn make_mat_lmb() -> Mat {
        Mat::Lmb(Lambertian::make_lmb(Vec3::make_vec3(0.4, 0.4, 0.4))) // Lambertian with a default albedo.
    }

    pub fn scatter(&self, target_ray: &Ray, normal: Vec3) -> Ray {
        match self {
            Mat::Lmb(tmp) => Lambertian::do_scatter(target_ray, normal),
        }
    }
}
