// This is mod entity.
// Hittables are entities.
#![allow(unused_variables)]
//------------------------------    Modules    -----------------------------------------

pub mod material;

use crate::entity::material::*;
use crate::graphics::ray;
use crate::math_support::*;
use crate::LIGHT_SPEED;
use ray::Ray;

//--------------------------------------------------------------------------------------
// Trait CanHit
pub trait CanHit {
    fn get_hit_time(&self, target_ray: &ray::Ray, min_tm: f64, max_tm: f64) -> f64 {
        -1.0 // Not hit
    }

    fn get_hit_normal(&self, pos: Vec3, tm: f64) -> Vec3;
}

//------------------------    Struct Plain    ------------------------------------------
pub struct Plain {
    // A xOz plain served as a background.
    y: f64,
    pub material: Mat,
}

impl Plain {
    pub fn make_plain(y: f64, material: Mat) -> Plain {
        Plain { y, material }
    }
}

impl CanHit for Plain {
    fn get_hit_time(&self, target_ray: &ray::Ray, min_tm: f64, max_tm: f64) -> f64 {
        let product: f64 = dot(
            target_ray.get_dir(),
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
        );
        if product < 0.0 {
            -1.0
        } else {
            let y_spd = -target_ray.get_dir().y * LIGHT_SPEED;
            let res = target_ray.get_tm() + (target_ray.get_pos().y - self.y) / y_spd;
            if res < min_tm || res > max_tm {
                -1.0
            } else {
                res
            }
        }
    }

    fn get_hit_normal(&self, pos: Vec3, tm: f64) -> Vec3 {
        Vec3::make_vec3(0.0, 1.0, 0.0)
    }
}

//-------------------------    Struct Sphere    ----------------------------------------
// Now the sphere is movable.
pub struct Sphere {
    centre: Vec3,
    r: f64,
    pub material: Mat,
    v: Vec3,
}

impl Sphere {
    pub fn make_sphere(centre: Vec3, r: f64, material: Mat, v: Vec3) -> Sphere {
        Sphere {
            centre,
            r,
            material,
            v,
        }
    }
    pub fn get_centre(&self, tm: f64) -> Vec3 {
        self.centre + tm * self.v
    }
    pub fn get_radius(&self) -> f64 {
        self.r
    }
}

impl CanHit for Sphere {
    fn get_hit_time(&self, target_ray: &Ray, min_tm: f64, max_tm: f64) -> f64 {
        let oc: Vec3 = target_ray.get_pos() - self.get_centre(target_ray.get_tm());
        let relative_spd = target_ray.get_dir() * LIGHT_SPEED - self.v;

        let a = dot(relative_spd, relative_spd);
        let b = dot(oc, relative_spd);
        let c = dot(oc, oc) - self.r * self.r;

        let delta = b.powf(2.0) - a * c;
        if delta < 0.0 {
            -1.0 // Not hit.
        } else {
            let mut take_time: f64 = (-b - delta.sqrt()) / a;
            if take_time < EPS {
                take_time = (-b + delta.sqrt()) / a;
            }
            if take_time < EPS {
                return -1.0;
            }
            let res: f64 = take_time + target_ray.get_tm();
            if res < min_tm || res > max_tm {
                -1.0
            } else {
                res
            }
        }
    }

    fn get_hit_normal(&self, pos: Vec3, tm: f64) -> Vec3 {
        let op: Vec3 = pos - self.get_centre(tm);
        op.normalize()
    }
}

//-------------------------------    Enum Entity    --------------------------------------

pub enum Entity {
    None,
    Pln(Plain),
    Sph(Sphere),
}

impl Entity {
    pub fn make_none_entity() -> Entity {
        Entity::None
    }

    pub fn scatter(&self, target_ray: &Ray, normal: Vec3) -> Ray {
        match self {
            Entity::Pln(tmp) => tmp.material.scatter(target_ray, normal),
            Entity::Sph(tmp) => tmp.material.scatter(target_ray, normal),
            _ => Ray::make_ray(crate::origin, Vec3::make_vec3(0.0, 1.0, 0.0), 0.0),
        }
    }

    pub fn get_albedo(&self) -> Vec3 {
        match self {
            Entity::Pln(tmp) => tmp.material.get_albedo(),
            Entity::Sph(tmp) => tmp.material.get_albedo(),
            _ => Vec3::make_vec3(0.0, 0.0, 0.0),
        }
    }
}

impl CanHit for Entity {
    fn get_hit_time(&self, target_ray: &Ray, min_tm: f64, max_tm: f64) -> f64 {
        match self {
            Entity::Pln(tmp) => tmp.get_hit_time(target_ray, min_tm, max_tm),
            Entity::Sph(tmp) => tmp.get_hit_time(target_ray, min_tm, max_tm),
            _ => -1.0,
        }
    }

    fn get_hit_normal(&self, pos: Vec3, tm: f64) -> Vec3 {
        match self {
            Entity::Pln(tmp) => tmp.get_hit_normal(pos, tm),
            Entity::Sph(tmp) => tmp.get_hit_normal(pos, tm),
            _ => Vec3::make_vec3(0.0, 1.0, 0.0),
        }
    }
}
