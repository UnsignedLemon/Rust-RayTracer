// This is mod entity.
// Hittables are entities.
#![allow(unused_variables)]
//------------------------------    Modules    -----------------------------------------

pub mod material;

use crate::entity::material::*;
use crate::graphics::ray;
use crate::math_support::*;
use crate::origin;
use ray::Ray;

//------------------------    Struct AABB    -------------------------------------------
#[derive(Clone)]
pub struct Aabb {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
}

impl Aabb {
    pub fn make_aabb(v0: Vec3, v1: Vec3) -> Aabb {
        Aabb {
            x0: min(v0.x, v1.x),
            x1: max(v0.x, v1.x),
            y0: min(v0.y, v1.y),
            y1: max(v0.y, v1.y),
            z0: min(v0.z, v1.z),
            z1: max(v0.z, v1.z),
        }
    }

    pub fn merge_aabb(v0: &Aabb, v1: &Aabb) -> Aabb {
        Aabb {
            x0: min(v0.x0, v1.x0),
            x1: max(v0.x1, v1.x1),
            y0: min(v0.y0, v1.y0),
            y1: max(v0.y1, v1.y1),
            z0: min(v0.z0, v1.z0),
            z1: max(v0.z1, v1.z1),
        }
    }

    pub fn can_hit(&self, target_ray: &ray::Ray) -> bool {
        let x_range: (f64, f64) = calc_time_range(
            self.x0,
            self.x1,
            target_ray.get_pos().x,
            target_ray.get_dir().x,
        );
        if x_range.1 < 0.0 {
            return false;
        }

        let y_range: (f64, f64) = calc_time_range(
            self.y0,
            self.y1,
            target_ray.get_pos().y,
            target_ray.get_dir().y,
        );
        if y_range.1 < 0.0 {
            return false;
        }

        let z_range: (f64, f64) = calc_time_range(
            self.z0,
            self.z1,
            target_ray.get_pos().z,
            target_ray.get_dir().z,
        );
        if z_range.1 < 0.0 {
            return false;
        }

        max(x_range.0, max(y_range.0, z_range.0)) < min(x_range.1, min(y_range.1, z_range.1))
    }
}

//--------------------------------------------------------------------------------------
// Trait CanHit
pub trait CanHit {
    fn get_hit_time(&self, target_ray: &ray::Ray) -> f64 {
        -1.0 // Not hit
    }

    fn get_hit_normal(&self, pos: Vec3, tm: f64) -> Vec3;

    fn gen_aabb(&self, t0: f64, t1: f64) -> Option<Aabb> {
        None
    }
}

//------------------------    Struct Plain    ------------------------------------------
#[derive(Clone)]
pub struct Plain {
    // A xOz plain served as a background.
    pub y: f64,
    pub material: Mat,
}

impl Plain {
    pub fn make_plain(y: f64, material: Mat) -> Plain {
        Plain { y, material }
    }
}

impl CanHit for Plain {
    fn get_hit_time(&self, target_ray: &ray::Ray) -> f64 {
        let product: f64 = dot(
            target_ray.get_dir(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        );
        if product < 0.0 {
            -1.0
        } else {
            let z_spd = -target_ray.get_dir().z;
            (target_ray.get_pos().z - self.y) / z_spd
        }
    }

    fn get_hit_normal(&self, pos: Vec3, tm: f64) -> Vec3 {
        Vec3::make_vec3(0.0, 0.0, 1.0)
    }
}

//-------------------------    Struct Sphere    ----------------------------------------

#[derive(Clone)]
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
    fn get_hit_time(&self, target_ray: &Ray) -> f64 {
        let oc: Vec3 = target_ray.get_pos() - self.get_centre(target_ray.get_tm());
        let dir = target_ray.get_dir();

        let a = 1.0;
        let b = dot(oc, dir);
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
                -1.0
            } else {
                take_time
            }
        }
    }

    fn get_hit_normal(&self, pos: Vec3, tm: f64) -> Vec3 {
        let op: Vec3 = pos - self.get_centre(tm);
        op.normalize()
    }

    fn gen_aabb(&self, t0: f64, t1: f64) -> Option<Aabb> {
        let r_vec = Vec3::make_vec3(self.r, self.r, self.r);
        let centre_t0 = self.get_centre(t0);
        let centre_t1 = self.get_centre(t1);
        let v1: Aabb = Aabb::make_aabb(centre_t0 - r_vec, centre_t0 + r_vec);
        let v2: Aabb = Aabb::make_aabb(centre_t1 - r_vec, centre_t1 + r_vec);

        Some(Aabb::merge_aabb(&v1, &v2))
    }
}

//---------------------------    Struct triangle    --------------------------------------

#[derive(Clone)]
pub struct Triangle {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
    pub inner_vertex: Vec3, // The outer normal points at the opposite of the inner vertex.s
    pub material: Mat,
    pub v: Vec3,
}

impl Triangle {
    pub fn make_triangle(
        v1: Vec3,
        v2: Vec3,
        v3: Vec3,
        inner_vertex: Vec3,
        material: Mat,
        v: Vec3,
    ) -> Triangle {
        Triangle {
            v1,
            v2,
            v3,
            inner_vertex,
            material,
            v,
        }
    }
}

impl CanHit for Triangle {
    fn get_hit_time(&self, target_ray: &ray::Ray) -> f64 {
        let tm = target_ray.get_tm();
        let normal = self.get_hit_normal(origin, 0.0);

        let n_v1 = at(self.v1, self.v, tm);
        let n_v2 = at(self.v2, self.v, tm);
        let n_v3 = at(self.v3, self.v, tm);

        let op = target_ray.get_pos() - n_v1;
        let spd = -dot(normal, target_ray.get_dir());
        let dist = dot(normal, op);

        if close_to(spd, 0.0) {
            -1.0
        } else {
            let tm = dist / spd;
            if tm < 0.0 {
                -1.0
            } else {
                let hit_pos = op + tm * target_ray.get_dir();
                let v1 = hit_pos;
                let v2 = n_v2 - n_v1;
                let v3 = n_v3 - n_v1;

                let mut det = v2.x * v3.y - v3.x * v2.y;
                let proj_a;
                let proj_b;
                if det != 0.0 {
                    proj_a = (v1.x * v3.y - v3.x * v1.y) / det;
                    proj_b = (v2.x * v1.y - v1.x * v2.y) / det;
                } else {
                    det = v2.y * v3.z - v3.y * v2.z;
                    if det != 0.0 {
                        proj_a = (v1.y * v3.z - v3.y * v1.z) / det;
                        proj_b = (v2.y * v1.z - v1.y * v2.z) / det;
                    } else {
                        det = v2.x * v3.z - v3.x * v2.z;
                        proj_a = (v1.x * v3.z - v3.x * v1.z) / det;
                        proj_b = (v2.x * v1.z - v1.x * v2.z) / det;
                    }
                }

                if (0.0..=1.0).contains(&proj_a)
                    && (0.0..=1.0).contains(&proj_b)
                    && proj_a + proj_b <= 1.0
                {
                    tm
                } else {
                    -1.0
                }
            }
        }
    }

    fn get_hit_normal(&self, pos: Vec3, tm: f64) -> Vec3 {
        // It doesn't change with time.
        let mut res: Vec3 = cross(self.v2 - self.v1, self.v3 - self.v1);
        res = res.normalize();
        if dot(res, self.inner_vertex - self.v1) > 0.0 {
            origin - res
        } else {
            res
        }
    }

    fn gen_aabb(&self, t0: f64, t1: f64) -> Option<Aabb> {
        let n0_v1 = at(self.v1, self.v, t0);
        let n0_v2 = at(self.v2, self.v, t0);
        let n0_v3 = at(self.v3, self.v, t0);
        let n1_v1 = at(self.v1, self.v, t1);
        let n1_v2 = at(self.v2, self.v, t1);
        let n1_v3 = at(self.v3, self.v, t1);

        let aabb1 = Aabb::merge_aabb(
            &Aabb::make_aabb(n0_v1, n0_v2),
            &Aabb::make_aabb(n0_v1, n0_v3),
        );
        let aabb2 = Aabb::merge_aabb(
            &Aabb::make_aabb(n1_v1, n1_v2),
            &Aabb::make_aabb(n1_v1, n1_v3),
        );

        Some(Aabb::merge_aabb(&aabb1, &aabb2))
    }
}

//-------------------------------    Enum Entity    ------------------------------------

#[derive(Clone)]
pub enum Entity {
    None,
    Pln(Plain),
    Sph(Sphere),
    Tri(Triangle),
}

impl Entity {
    pub fn make_none_entity() -> Entity {
        Entity::None
    }

    pub fn get_emission(&self) -> Vec3 {
        match self {
            Entity::Pln(tmp) => tmp.material.get_light_color(),
            Entity::Sph(tmp) => tmp.material.get_light_color(),
            Entity::Tri(tmp) => tmp.material.get_light_color(),
            Entity::None => origin,
        }
    }

    pub fn scatter(&self, target_ray: &Ray, normal: Vec3) -> Ray {
        match self {
            Entity::Pln(tmp) => tmp.material.scatter(target_ray, normal),
            Entity::Sph(tmp) => tmp.material.scatter(target_ray, normal),
            Entity::Tri(tmp) => tmp.material.scatter(target_ray, normal),
            _ => Ray::make_ray(crate::origin, Vec3::make_vec3(0.0, 1.0, 0.0), 0.0),
        }
    }

    pub fn get_albedo(&self) -> Vec3 {
        match self {
            Entity::Pln(tmp) => tmp.material.get_albedo(),
            Entity::Sph(tmp) => tmp.material.get_albedo(),
            Entity::Tri(tmp) => tmp.material.get_albedo(),
            _ => Vec3::make_vec3(0.0, 0.0, 0.0),
        }
    }
}

impl CanHit for Entity {
    fn get_hit_time(&self, target_ray: &Ray) -> f64 {
        match self {
            Entity::Pln(tmp) => tmp.get_hit_time(target_ray),
            Entity::Sph(tmp) => tmp.get_hit_time(target_ray),
            Entity::Tri(tmp) => tmp.get_hit_time(target_ray),
            _ => -1.0,
        }
    }

    fn get_hit_normal(&self, pos: Vec3, tm: f64) -> Vec3 {
        match self {
            Entity::Pln(tmp) => tmp.get_hit_normal(pos, tm),
            Entity::Sph(tmp) => tmp.get_hit_normal(pos, tm),
            Entity::Tri(tmp) => tmp.get_hit_normal(pos, tm),
            _ => Vec3::make_vec3(0.0, 1.0, 0.0),
        }
    }

    fn gen_aabb(&self, t0: f64, t1: f64) -> Option<Aabb> {
        match self {
            Entity::Sph(tmp) => tmp.gen_aabb(t0, t1),
            Entity::Tri(tmp) => tmp.gen_aabb(t0, t1),
            _ => None,
        }
    }
}

//----------------------    Implementations for sort    --------------------------------

use std::cmp;
use std::cmp::Ordering;

impl cmp::PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Entity::None => *other == Entity::None,
            Entity::Pln(cur) => match other {
                Entity::Pln(oth) => cur.y == oth.y,
                _ => false,
            },
            Entity::Sph(cur) => match other {
                Entity::Sph(oth) => cur.get_centre(0.0).x == oth.get_centre(0.0).x,
                _ => false,
            },
            Entity::Tri(cur) => match other {
                Entity::Tri(oth) => cur.v1.x == oth.v1.x,
                _ => false,
            },
        }
    }
}

impl cmp::Eq for Entity {}

impl cmp::Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Entity::None => match other {
                Entity::None => Ordering::Equal,
                _ => Ordering::Less,
            },
            Entity::Pln(cur) => match other {
                Entity::None => Ordering::Greater,
                Entity::Pln(oth) => (cur.y).partial_cmp(&oth.y).unwrap(),
                _ => Ordering::Less,
            },
            Entity::Sph(cur) => match other {
                Entity::Sph(oth) => (cur.get_centre(0.0).x)
                    .partial_cmp(&oth.get_centre(0.0).x)
                    .unwrap(),
                Entity::Tri(oth) => Ordering::Less,
                _ => Ordering::Greater,
            },
            Entity::Tri(cur) => match other {
                Entity::Tri(oth) => (cur.v1.x).partial_cmp(&oth.v1.x).unwrap(),
                _ => Ordering::Greater,
            },
        }
    }
}

impl cmp::PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
