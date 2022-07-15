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

fn rotate(val: Vec3, f: f64) -> Vec3 {
    let sin = f.sin();
    let cos = f.cos();
    Vec3::make_vec3(val.x * cos - val.y * sin, val.x * sin + val.y * cos, val.z)
}

impl World {
    fn gen_rec(
        obj_list: &mut Vec<Entity>,
        pos: Vec3,
        edg: (Vec3, Vec3, Vec3),
        mmat: Mat,
        vv: Vec3,
    ) {
        let v1 = pos;
        let v2 = pos + edg.0;
        let v4 = pos + edg.1;
        let v3 = v2 + edg.1;
        let v5 = v1 + edg.2;
        let v6 = v2 + edg.2;
        let v7 = v3 + edg.2;
        let v8 = v4 + edg.2;

        let inner_vertex = (v1 + v7) / 2.0;

        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v1,
            v2,
            v4,
            inner_vertex,
            mmat.clone(),
            vv,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v3,
            v2,
            v4,
            inner_vertex,
            mmat.clone(),
            vv,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v5,
            v6,
            v8,
            inner_vertex,
            mmat.clone(),
            vv,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v7,
            v6,
            v8,
            inner_vertex,
            mmat.clone(),
            vv,
        )));

        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v2,
            v3,
            v6,
            inner_vertex,
            mmat.clone(),
            vv,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v7,
            v3,
            v6,
            inner_vertex,
            mmat.clone(),
            vv,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v1,
            v4,
            v5,
            inner_vertex,
            mmat.clone(),
            vv,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v8,
            v4,
            v5,
            inner_vertex,
            mmat.clone(),
            vv,
        )));

        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v1,
            v2,
            v5,
            inner_vertex,
            mmat.clone(),
            vv,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v6,
            v2,
            v5,
            inner_vertex,
            mmat.clone(),
            vv,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v4,
            v3,
            v8,
            inner_vertex,
            mmat.clone(),
            vv,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v7,
            v3,
            v8,
            inner_vertex,
            mmat,
            vv,
        )));
    }

    fn gen(obj_list: &mut Vec<Entity>, id: f64) {
        let l1 = 2.2;
        let z1 = 0.5;
        let l2 = 3.0;
        let z2 = 0.7;
        let l3 = 8.0;
        let z3 = 1.2;

        let f = std::f64::consts::PI / 2.5 * id;

        let tan36 = 0.726;
        let v_ = Vec3::make_vec3(-0.0, 0.0, -30.0);

        let v0 = rotate(Vec3::make_vec3(0.0, 0.0, -8.0), f);
        let v1 = rotate(Vec3::make_vec3(0.0, -1.0, 1.0), f);
        let v2 = rotate(Vec3::make_vec3(-l1 * tan36, -l1, z1), f);
        let v3 = rotate(Vec3::make_vec3(l1 * tan36, -l1, z1), f);
        let v4 = rotate(Vec3::make_vec3(-l2 * tan36, -l2, z2), f);
        let v5 = rotate(Vec3::make_vec3(l2 * tan36, -l2, z2), f);
        let v6 = rotate(Vec3::make_vec3(0.0, -l3, z3), f);
        let inner = rotate(Vec3::make_vec3(0.0, -3.0, 0.0), f);

        let mmat2: Mat = Mat::make_mat_mtl(0.3, 0.2, 0.8, 0.0);
        let mmat: Mat = Mat::make_mat_detc(0.2, 0.3, 0.8, 2.2);
        let mmat3: Mat = Mat::make_mat_detc(0.6, 0.6, 0.8, 3.2);

        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v0,
            v1,
            v2,
            inner,
            mmat2.clone(),
            origin,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v0,
            v1,
            v3,
            inner,
            mmat2.clone(),
            origin,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v1,
            v2,
            v4,
            inner,
            mmat2.clone(),
            origin,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v1, v3, v5, inner, mmat2, origin,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v4,
            v1,
            v6,
            inner,
            mmat.clone(),
            origin,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v5, v1, v6, inner, mmat3, origin,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v0,
            v4,
            v6,
            inner,
            mmat.clone(),
            origin,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v0,
            v5,
            v6,
            inner,
            mmat.clone(),
            origin,
        )));
        obj_list.push(Entity::Tri(Triangle::make_triangle(
            v4, v5, v_, inner, mmat, origin,
        )));
    }

    pub fn make_world() -> World {
        let bg: Entity = Entity::make_none_entity();
        //Entity::Pln(Plain::make_plain(-13.0, Mat::make_mat_mtl(0.0, 0.0, 0.05,0.0)));

        let mut new_list: Vec<Entity> = vec![
            Entity::Sph(Sphere::make_sphere(
                origin + Vec3::make_vec3(0.0, 80.0, 0.0),
                28.0,
                Mat::make_mat_lghtsrc(0.0, 0.0, 0.8),
                origin,
            )),
            Entity::Sph(Sphere::make_sphere(
                origin + Vec3::make_vec3(0.0, -50.0, 0.0),
                28.0,
                Mat::make_mat_lghtsrc(0.0, 0.0, 0.8),
                origin,
            )), /*
                Entity::Sph(Sphere::make_sphere(
                    origin + Vec3::make_vec3(0.0, 20.0, 18.0),
                    10.0,
                    Mat::make_mat_lghtsrc(1.0, 1.0, 1.0),
                    origin,
                )),
                Entity::Sph(Sphere::make_sphere(
                    origin + Vec3::make_vec3(0.0, -20.0, 18.0),
                    10.0,
                    Mat::make_mat_lghtsrc(1.0, 1.0, 1.0),
                    origin,
                )),*/
            Entity::Sph(Sphere::make_sphere(
                origin + Vec3::make_vec3(0.0, 0.0, 2.0),
                0.4,
                Mat::make_mat_lghtsrc(1.0, 0.0, 0.0),
                origin,
            )),
            Entity::Sph(Sphere::make_sphere(
                origin + Vec3::make_vec3(0.0, 0.0, 2.0),
                0.45,
                Mat::make_mat_detc(1.0, 0.8, 0.8, 1.5),
                origin,
            )),
            Entity::Sph(Sphere::make_sphere(
                origin + Vec3::make_vec3(0.0, 0.0, 0.0),
                0.8,
                Mat::make_mat_detc(0.8, 0.5, 0.4, 12.2),
                origin,
            )),
        ];

        World::gen(&mut new_list, 0.0);
        World::gen(&mut new_list, 1.0);
        World::gen(&mut new_list, 2.0);
        World::gen(&mut new_list, 3.0);
        World::gen(&mut new_list, 4.0);

        for id in 0..5 {
            let f = std::f64::consts::PI / 2.5 * id as f64;
            let f2 = std::f64::consts::PI / 5.0 + f;
            let g = std::f64::consts::PI / 8.0;
            let f = f;
            let f2 = f2;

            let spd: Vec3 = Vec3::make_vec3(0.0, 0.0, 0.0);
            World::gen_rec(
                &mut new_list,
                rotate(Vec3::make_vec3(-0.25, -2.0, 2.5), f),
                (
                    rotate(Vec3::make_vec3(0.5, 0.0, 0.0), f),
                    rotate(Vec3::make_vec3(0.0, -0.5, 0.0), f),
                    rotate(Vec3::make_vec3(0.0, 0.0, 1.7), f),
                ),
                Mat::make_mat_detc(0.7, 0.7, 0.9, 3.3),
                rotate(spd, f),
            );
            World::gen_rec(
                &mut new_list,
                rotate(Vec3::make_vec3(-0.3, -3.4, 2.0), f2),
                (
                    rotate(Vec3::make_vec3(0.6, 0.0, 0.0), f2),
                    rotate(Vec3::make_vec3(0.0, -0.2, 0.0), f2),
                    rotate(Vec3::make_vec3(0.0, 0.0, 4.3), f2),
                ),
                Mat::make_mat_mtl(0.8, 0.8, 1.0, 0.0),
                rotate(spd, f2),
            );
        }

        World::gen_rec(
            &mut new_list,
            Vec3::make_vec3(-75.0, -75.0, 20.0),
            (
                Vec3::make_vec3(150.0, 0.0, 0.0),
                Vec3::make_vec3(0.0, 150.0, 0.0),
                Vec3::make_vec3(0.0, 0.0, 1.0),
            ),
            Mat::make_mat_lghtsrc(1.0, 1.0, 1.0),
            origin,
        );
        /*
        World::gen_rec(&mut new_list,
                        Vec3::make_vec3(-0.2,-0.2,2.0),
                        (Vec3::make_vec3(0.4,0.0,0.0),
                        Vec3::make_vec3(0.0,0.4,0.0),
                        Vec3::make_vec3(0.0,0.0,40.0),),
                        Mat::make_mat_lghtsrc(1.0,0.2,0.2),
                        Vec3::make_vec3(5.0,5.0,0.0),
        );		*/

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
            Vec3::make_vec3(0.001, 0.001, 0.0005)
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
