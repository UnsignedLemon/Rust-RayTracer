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
#[derive(Clone)]
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
        Ray::make_ray(target_ray.get_pos(), new_dir, target_ray.get_tm())
    }
}

//-------------------------------    Struct Metal    -------------------------------------
#[derive(Clone)]
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

        Ray::make_ray(target_ray.get_pos(), new_dir, target_ray.get_tm())
    }
}

//---------------------------    Struct Dielectric    ------------------------------------
#[derive(Clone)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn make_detc(ir: f64) -> Dielectric {
        Dielectric { ir }
    }

    fn reflectance(cos_theta: f64, ratio: f64) -> f64 {
        let r0: f64 = (1.0 - ratio) / (1.0 + ratio);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
    }
}

impl Scatter for Dielectric {
    fn do_scatter(&self, target_ray: &Ray, normal: Vec3) -> Ray {
        let normal = normal.normalize();
        let dir = target_ray.get_dir().normalize();

        let refraction_ratio = if is_front_face(dir, normal) {
            1.0 / self.ir
        } else {
            self.ir
        };
        let mut cos_theta = dot(dir, normal);
        if cos_theta < 0.0 {
            cos_theta = -cos_theta;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;

        let new_dir: Vec3 = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand_0_1()
        {
            reflect(dir, normal)
        } else {
            refract(dir, normal, refraction_ratio)
        };

        Ray::make_ray(target_ray.get_pos(), new_dir, target_ray.get_tm())
    }
}

//---------------------------    Enum for Materials    -----------------------------------
#[derive(Clone)]
pub enum Mat {
    Lmb(Lambertian),
    Mtl(Metal),
    Detc(Dielectric),
}

impl Mat {
    pub fn make_mat_lmb(x: f64, y: f64, z: f64) -> Mat {
        Mat::Lmb(Lambertian::make_lmb(Vec3::make_vec3(x, y, z)))
    }

    pub fn make_mat_mtl(x: f64, y: f64, z: f64, fuzz: f64) -> Mat {
        // Albedo with fuzz.
        Mat::Mtl(Metal::make_mtl(Vec3::make_vec3(x, y, z), fuzz))
    }

    pub fn make_mat_detc(ir: f64) -> Mat {
        // Index of refraction only.
        Mat::Detc(Dielectric::make_detc(ir))
    }

    pub fn scatter(&self, target_ray: &Ray, normal: Vec3) -> Ray {
        match self {
            Mat::Lmb(tmp) => tmp.do_scatter(target_ray, normal),
            Mat::Mtl(tmp) => tmp.do_scatter(target_ray, normal),
            Mat::Detc(tmp) => tmp.do_scatter(target_ray, normal),
        }
    }

    pub fn get_albedo(&self) -> Vec3 {
        match self {
            Mat::Lmb(tmp) => tmp.albedo,
            Mat::Mtl(tmp) => tmp.albedo,
            Mat::Detc(tmp) => Vec3::make_vec3(1.0, 1.0, 1.0), // Pure glass.
        }
    }
}
