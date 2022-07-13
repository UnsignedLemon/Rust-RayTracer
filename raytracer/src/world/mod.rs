// Struct World to store all objs.
#![allow(unused_variables)]

use crate::entity::material::*;
use crate::entity::Aabb;
use crate::entity::*;
use crate::graphics::ray::Ray;
use crate::math_support::*;

use Option;

use crate::flash_t0;
use crate::flash_t1;
use crate::origin;
use crate::ITERATION_DEPTH;

//-------------------------------    Struct Bvh node    ----------------------------------
#[derive(Clone)]
pub struct Node {
    pub lch: Option<Box<Node>>,
    pub rch: Option<Box<Node>>,
    pub hit_box: Option<Aabb>,
    pub binding_obj: Option<Entity>,
}

impl Node {
    pub fn make_node() -> Node {
        Node {
            lch: None,
            rch: None,
            hit_box: None,
            binding_obj: None,
        }
    }

    pub fn bind_entity(&mut self, obj: Entity) {
        self.hit_box = obj.gen_aabb(flash_t0, flash_t1);
        self.binding_obj = Some(obj);
    }
}

//-----------------------    Build & Find in bvh    ------------------------------------
fn build_bvh(mut obj_list: Vec<Entity>) -> Option<Box<Node>> {
    match obj_list.len() {
        0 => Some(Box::new(Node::make_node())),
        1 => {
            let mut cur: Node = Node::make_node();
            cur.bind_entity(obj_list[0].clone());
            Some(Box::new(cur))
        }
        _ => {
            obj_list.sort();
            let mid = obj_list.len() / 2;

            let mut lvec = vec![];
            for ele in obj_list.iter().take(mid) {
                lvec.push(ele.clone());
            }

            let mut rvec = vec![];
            for ele in obj_list.iter().skip(mid) {
                rvec.push(ele.clone());
            }

            let mut cur: Node = Node::make_node();
            cur.lch = build_bvh(lvec);
            cur.rch = build_bvh(rvec);
            cur.binding_obj = None;
            let laabb: Aabb = cur.lch.as_ref().unwrap().clone().hit_box.unwrap();
            let raabb: Aabb = cur.rch.as_ref().unwrap().clone().hit_box.unwrap();
            cur.hit_box = Some(Aabb::merge_aabb(&laabb, &raabb));

            Some(Box::new(cur))
        }
    }
}

fn just_hit_it(cur: &Node, target_ray: &Ray) -> (Entity, f64) {
    match &cur.binding_obj {
        Some(obj) => {
            // It's a leaf node and there's an obj to hit.
            let tm: f64 = obj.get_hit_time(target_ray);
            if tm > EPS {
                (obj.clone(), tm)
            } else {
                (Entity::make_none_entity(), -1.0)
            }
        }
        None => {
            if let Some(bx) = &cur.hit_box {
                if !bx.can_hit(target_ray) {
                    return (Entity::make_none_entity(), -1.0);
                }
            }
            let mut l_rec = (Entity::make_none_entity(), -1.0);
            let mut r_rec = (Entity::make_none_entity(), -1.0);
            if let Some(l) = &cur.lch {
                l_rec = just_hit_it(l, target_ray);
            }
            if let Some(r) = &cur.rch {
                r_rec = just_hit_it(r, target_ray);
            }

            if l_rec.1 < EPS {
                return r_rec;
            }
            if r_rec.1 < EPS {
                return l_rec;
            }

            if l_rec.1 < r_rec.1 {
                l_rec
            } else {
                r_rec
            }
        }
    }
}

//-------------------------------    Struct World    -------------------------------------

pub struct World {
    pub bg: Entity,
    bvh_root: Node,
}

impl World {
    pub fn make_world() -> World {
        let bg: Entity = Entity::Pln(Plain::make_plain(-0.3, Mat::make_mat_lmb(0.7, 0.6, 0.5)));

        let mut new_list: Vec<Entity> = vec![
            Entity::Sph(Sphere::make_sphere(
                origin,
                0.3,
                Mat::make_mat_lghtsrc(1.0, 0.7, 1.0),
                origin,
            )),
            Entity::Sph(Sphere::make_sphere(
                origin + Vec3::make_vec3(0.6, 0.0, 0.0),
                0.3,
                Mat::make_mat_mtl(0.8, 0.8, 0.96, 0.3),
                origin,
            )),
            Entity::Sph(Sphere::make_sphere(
                origin + Vec3::make_vec3(-0.6, 0.0, 0.0),
                0.3,
                Mat::make_mat_detc(1.5),
                origin,
            )),
            Entity::Sph(Sphere::make_sphere(
                origin + Vec3::make_vec3(-0.6, 0.0, 0.0),
                0.25,
                Mat::make_mat_detc(1.0 / 1.5),
                origin,
            )),
        ];

        let size_per_cell: f64 = 0.15;
        let ball_radius: f64 = 0.04;
        let max_offset: f64 = size_per_cell - 2.0 * ball_radius;

        for i in -11..21 {
            for j in -11..21 {
                let do_or_not = rand_0_1();
                if do_or_not > 0.4 {
                    continue;
                }

                let x: f64 = (i as f64) * size_per_cell + rand_0_1() * max_offset;
                let y: f64 = ball_radius + rand_0_1() * ball_radius - 0.3;
                let z: f64 = (j as f64) * size_per_cell + rand_0_1() * max_offset;
                let target_pos: Vec3 = Vec3::make_vec3(x, y, z);

                if dist(target_pos, origin) < 0.3 + ball_radius
                    || dist(target_pos, Vec3::make_vec3(0.6, 0.0, 0.0)) < 0.3 + ball_radius
                    || dist(target_pos, Vec3::make_vec3(-0.6, 0.0, 0.0)) < 0.3 + ball_radius
                {
                    continue;
                }

                let rand_material = rand_0_1();
                let rand_albedo = Vec3::make_vec3(rand_0_1(), rand_0_1(), rand_0_1());

                let rand_spd = Vec3::make_vec3(0.0, rand_0_1() * 0.07, 0.0);

                if rand_material > 0.85 {
                    new_list.push(Entity::Sph(Sphere::make_sphere(
                        target_pos,
                        ball_radius,
                        Mat::make_mat_lghtsrc(
                            rand_0_1() * 0.5 + 0.5,
                            rand_0_1() * 0.5 + 0.5,
                            rand_0_1() * 0.5 + 0.5,
                        ),
                        origin,
                    )));
                }

                if rand_material > 0.75 {
                    new_list.push(Entity::Sph(Sphere::make_sphere(
                        target_pos,
                        ball_radius,
                        Mat::make_mat_detc(1.5),
                        rand_spd,
                    )));
                } else if rand_material > 0.6 {
                    new_list.push(Entity::Sph(Sphere::make_sphere(
                        target_pos,
                        ball_radius,
                        Mat::make_mat_mtl(rand_albedo.x, rand_albedo.y, rand_albedo.z, rand_0_1()),
                        rand_spd,
                    )));
                } else {
                    new_list.push(Entity::Sph(Sphere::make_sphere(
                        target_pos,
                        ball_radius,
                        Mat::make_mat_lmb(rand_albedo.x, rand_albedo.y, rand_albedo.z),
                        rand_spd,
                    )));
                }
            }
        }

        let rt = build_bvh(new_list);
        World {
            bg,
            bvh_root: rt.unwrap().as_ref().clone(),
        }
    }

    fn do_trace(&self, target_ray: &Ray, depth: i32) -> Vec3 {
        if depth < 0 {
            return Vec3::make_vec3(0.0, 0.0, 0.0);
        }
        let cur_tm = target_ray.get_tm();

        let mut first_hit_time: f64 = (&self.bg).get_hit_time(target_ray);
        let mut target_obj = &self.bg;

        let hit_rec = just_hit_it(&self.bvh_root, target_ray);
        let hit_obj = hit_rec.0;
        let hit_obj_time = hit_rec.1;

        if first_hit_time < EPS || (hit_obj_time > EPS && hit_obj_time < first_hit_time) {
            first_hit_time = hit_obj_time;
            target_obj = &hit_obj;
        }

        if first_hit_time < 0.0 {
            Vec3::make_vec3(0.15, 0.15, 0.18)
        //    let p: f64 = 0.5 * (target_ray.get_dir().y + 1.0);
        //    (1.0 - p) * Vec3::make_vec3(1.0, 1.0, 1.0) + p * Vec3::make_vec3(0.5, 0.7, 1.0)
        } else {
            let target_color: Vec3 = target_obj.get_emission();
            if target_color.x == 0.0 && target_color.y == 0.0 && target_color.z == 0.0 {
                let pos: Vec3 = target_ray.get_pos() + first_hit_time * target_ray.get_dir();
                let normal: Vec3 = target_obj.get_hit_normal(pos, cur_tm);
                let target_ray = &(Ray::make_ray(pos, target_ray.get_dir(), cur_tm));
                let target_ray = &(target_obj.scatter(target_ray, normal));
                (target_obj.get_albedo()) * self.do_trace(target_ray, depth - 1)
            } else {
                target_color
            }
        }
    }

    // Here comes the most important function that actually do the tracing process of target ray.
    pub fn trace_ray_color(&self, target_ray: &Ray) -> Vec3 {
        self.do_trace(target_ray, ITERATION_DEPTH)
    }
}
