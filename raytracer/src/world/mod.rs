// Struct World to store all objs.
#![allow(unused_variables)]

use crate::entity::material::*;
use crate::entity::*;
use crate::graphics::ray::Ray;
use crate::math_support::*;

use crate::math_support::EPS;
use crate::origin;
use crate::ITERATION_DEPTH;

//-------------------------------    Struct World    -------------------------------------

pub struct World {
    pub obj_list: Vec<Entity>,
}

impl World {
    pub fn make_world() -> World {
        let mut new_list: Vec<Entity> = vec![
            Entity::Pln(Plain::make_plain(-0.3, Mat::make_mat_lmb(0.5, 0.7, 0.6))),
            Entity::Sph(Sphere::make_sphere(
                origin,
                0.3,
                Mat::make_mat_lmb(0.5, 0.4, 0.4),
            )),
            Entity::Sph(Sphere::make_sphere(
                origin + Vec3::make_vec3(0.6, 0.0, 0.0),
                0.3,
                Mat::make_mat_mtl(0.8, 0.8, 0.96, 0.3),
            )),
            Entity::Sph(Sphere::make_sphere(
                origin + Vec3::make_vec3(-0.6, 0.0, 0.0),
                0.3,
                Mat::make_mat_detc(1.5),
            )),
            Entity::Sph(Sphere::make_sphere(
                origin + Vec3::make_vec3(-0.6, 0.0, 0.0),
                0.25,
                Mat::make_mat_detc(1.0 / 1.5),
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
                let z: f64 = (j as f64) * size_per_cell + rand_0_1() * max_offset;
                let target_pos: Vec3 = Vec3::make_vec3(x, ball_radius - 0.3, z);
                if dist(target_pos, origin) < 0.3 + ball_radius
                    || dist(target_pos, Vec3::make_vec3(0.6, 0.0, 0.0)) < 0.3 + ball_radius
                    || dist(target_pos, Vec3::make_vec3(-0.6, 0.0, 0.0)) < 0.3 + ball_radius
                {
                    continue;
                }

                let rand_material = rand_0_1();
                let rand_albedo = Vec3::make_vec3(rand_0_1(), rand_0_1(), rand_0_1());

                if rand_material > 0.9 {
                    new_list.push(Entity::Sph(Sphere::make_sphere(
                        target_pos,
                        ball_radius,
                        Mat::make_mat_detc(1.5),
                    )));
                } else if rand_material > 0.8 {
                    new_list.push(Entity::Sph(Sphere::make_sphere(
                        target_pos,
                        ball_radius,
                        Mat::make_mat_mtl(rand_albedo.x, rand_albedo.y, rand_albedo.z, rand_0_1()),
                    )));
                } else {
                    new_list.push(Entity::Sph(Sphere::make_sphere(
                        target_pos,
                        ball_radius,
                        Mat::make_mat_lmb(rand_albedo.x, rand_albedo.y, rand_albedo.z),
                    )));
                }
            }
        }

        World { obj_list: new_list }
    }

    fn do_trace(&self, target_ray: &Ray, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::make_vec3(0.0, 0.0, 0.0);
        }
        let mut target_obj = &(Entity::make_none_entity());
        let mut first_hit_time: f64 = -1.0;
        for obj in &(self.obj_list) {
            let tm: f64 = obj.get_hit_time(target_ray);
            if tm < EPS {
                continue;
            }
            if first_hit_time < EPS || first_hit_time > tm {
                first_hit_time = tm;
                target_obj = obj;
            }
        }

        if first_hit_time < EPS {
            // Hit nothing, background color.
            let p: f64 = 0.5 * (target_ray.get_dir().y + 1.0);
            (1.0 - p) * Vec3::make_vec3(1.0, 1.0, 1.0) + p * Vec3::make_vec3(0.5, 0.7, 1.0)
        } else {
            // To be updated with different materials.
            let pos: Vec3 = target_ray.get_pos() + first_hit_time * target_ray.get_dir();
            let normal: Vec3 = target_obj.get_hit_normal(pos);
            let target_ray = &(Ray::make_ray(pos, target_ray.get_dir()));
            let target_ray = &(target_obj.scatter(target_ray, normal));

            //return crate::DEFAULT_COLOR;
            (target_obj.get_albedo()) * self.do_trace(target_ray, depth - 1)
        }
    }

    // Here comes the most important function that actually do the tracing process of target ray.
    pub fn trace_ray_color(&self, target_ray: &Ray) -> Vec3 {
        self.do_trace(target_ray, ITERATION_DEPTH)
    }
}
