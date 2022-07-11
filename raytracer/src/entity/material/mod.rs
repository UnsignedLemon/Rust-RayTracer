// Mod Material
#![allow(unused_variables)]

use crate::graphics::ray::Ray;
use crate::math_support::*;

//--------------------------    Trait Scatter    ----------------------------------------
pub trait Scatter {
    fn do_scatter(&self, target_ray: &Ray, normal: Vec3) -> Ray;
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
    fn do_scatter(&self, target_ray: &Ray, normal: Vec3) -> Ray {
        let mut normal = normal.normalize();
        if dot(target_ray.get_dir(), normal) > -EPS {
            normal = crate::origin - normal;
        }
        let mut new_dir: Vec3 = normal + rand_normalized_vec();
        if close_to(new_dir.get_len(), 0.0) {
            new_dir = normal;
        }
        Ray::make_ray(target_ray.get_pos(), new_dir)
    }
}

//-------------------------------    Struct Metal    -------------------------------------

pub struct Metal {
    pub albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn make_mtl(albedo: Vec3, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Scatter for Metal {
    fn do_scatter(&self, target_ray: &Ray, normal: Vec3) -> Ray {
        let normal = normal.normalize();
        let mut new_dir: Vec3 =
            reflect(target_ray.get_dir(), normal) + self.fuzz * rand_normalized_vec();
        if close_to(new_dir.get_len(), 0.0) {
            new_dir = normal;
        }

        Ray::make_ray(target_ray.get_pos(), new_dir)
    }
}

//---------------------------    Struct Metal    -----------------------------------------

//---------------------------    Enum for Materials    -----------------------------------
pub enum Mat {
    Lmb(Lambertian),
    Mtl(Metal),
}

impl Mat {
    pub fn make_mat_lmb(x: f64, y: f64, z: f64) -> Mat {
        Mat::Lmb(Lambertian::make_lmb(Vec3::make_vec3(x, y, z)))
    }

    pub fn make_mat_mtl(x: f64, y: f64, z: f64, fuzz: f64) -> Mat {
        // Albedo with fuzz.
        Mat::Mtl(Metal::make_mtl(Vec3::make_vec3(x, y, z), fuzz))
    }

    pub fn scatter(&self, target_ray: &Ray, normal: Vec3) -> Ray {
        match self {
            Mat::Lmb(tmp) => tmp.do_scatter(target_ray, normal),
            Mat::Mtl(tmp) => tmp.do_scatter(target_ray, normal),
        }
    }

    pub fn get_albedo(&self) -> Vec3 {
        match self {
            Mat::Lmb(tmp) => tmp.albedo,
            Mat::Mtl(tmp) => tmp.albedo,
        }
    }
}
