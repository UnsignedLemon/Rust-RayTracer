// This is mod entity.
// Hittables are entities.
#![allow(unused_variables)]
//------------------------------    Modules    -----------------------------------------

pub mod material;

use crate::entity::material::*;
use crate::graphics::ray;
use crate::math_support::*;
use crate::DEFAULT_COLOR;
use ray::Ray;

//--------------------------------------------------------------------------------------
// Trait CanHit
pub trait CanHit {
    fn get_hit_time(&self, target_ray: &ray::Ray) -> f64 {
        -1.0 // Not hit
    }

    fn get_hit_color(&self, target_ray: &Ray) -> Vec3 {
        DEFAULT_COLOR // Default color: color missing.
    }

    fn get_hit_normal(&self, pos: Vec3) -> Vec3;
}

//------------------------    Struct Plain    ------------------------------------------
pub struct Plain {
    // A xOz plain served as a background.
    y: f64,
    pub material: Mat,
}

impl Plain {
    pub fn make_plain(y: f64) -> Plain {
        Plain {
            y,
            material: Mat::make_mat_lmb(),
        }
    }
}

impl CanHit for Plain {
    fn get_hit_time(&self, target_ray: &ray::Ray) -> f64 {
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
            let y_spd = -target_ray.get_dir().y;
            (target_ray.get_pos().y - self.y) / y_spd
        }
    }

    fn get_hit_color(&self, target_ray: &Ray) -> Vec3 {
        Vec3::make_vec3(0.7, 0.7, 0.9)
    }

    fn get_hit_normal(&self, pos: Vec3) -> Vec3 {
        Vec3::make_vec3(0.0, 1.0, 0.0)
    }
}

//-------------------------    Struct Sphere    ----------------------------------------

pub struct Sphere {
    centre: Vec3,
    r: f64,
    pub material: Mat,
}

impl Sphere {
    pub fn make_sphere(centre: Vec3, r: f64) -> Sphere {
        Sphere {
            centre,
            r,
            material: Mat::make_mat_lmb(),
        }
    }
    pub fn get_centre(&self) -> Vec3 {
        self.centre
    }
    pub fn get_radius(&self) -> f64 {
        self.r
    }
}

impl CanHit for Sphere {
    fn get_hit_time(&self, target_ray: &Ray) -> f64 {
        let oc: Vec3 = target_ray.get_pos() - self.centre;

        let b = dot(oc, target_ray.get_dir());
        let c = dot(oc, oc) - self.r * self.r;

        let delta = b * b - c;
        if delta < 0.0 {
            -1.0 // Not hit.
        } else {
            -b - delta.sqrt() // First hit time.
        }
    }

    fn get_hit_color(&self, target_ray: &Ray) -> Vec3 {
        let tm: f64 = self.get_hit_time(&target_ray);
        let hit_pos: Vec3 = target_ray.get_pos() + tm * target_ray.get_dir();
        let hit_pos = hit_pos - self.centre;
        hit_pos.normalize()  // As the color of the surface.
    }

    fn get_hit_normal(&self, pos: Vec3) -> Vec3 {
        let op: Vec3 = pos - self.centre;
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
            _ => Ray::make_ray(crate::origin, Vec3::make_vec3(0.0, 1.0, 0.0)),
        }
    }
}

impl CanHit for Entity {
    fn get_hit_time(&self, target_ray: &Ray) -> f64 {
        match self {
            Entity::Pln(tmp) => tmp.get_hit_time(target_ray),
            Entity::Sph(tmp) => tmp.get_hit_time(target_ray),
            _ => -1.0,
        }
    }

    fn get_hit_color(&self, target_ray: &Ray) -> Vec3 {
        match self {
            Entity::Pln(tmp) => tmp.get_hit_color(target_ray),
            Entity::Sph(tmp) => tmp.get_hit_color(target_ray),
            _ => crate::DEFAULT_COLOR,
        }
    }

    fn get_hit_normal(&self, pos: Vec3) -> Vec3 {
        match self {
            Entity::Pln(tmp) => tmp.get_hit_normal(pos),
            Entity::Sph(tmp) => tmp.get_hit_normal(pos),
            _ => Vec3::make_vec3(0.0, 1.0, 0.0),
        }
    }
}
