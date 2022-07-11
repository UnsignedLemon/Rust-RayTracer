// Struct World to store all objs.
#![allow(unused_variables)]

use crate::entity::*;
use crate::graphics::ray::Ray;
use crate::math_support::*;

use crate::math_support::EPS;
use crate::origin;
use crate::ITERATION_DEPTH;
use crate::VIEWPORT_DEPTH;

//-------------------------------    Struct World    -------------------------------------

pub struct World {
    pub obj_list: Vec<Entity>,
}

impl World {
    pub fn make_world() -> World {
        let new_list: Vec<Entity> = vec![
       		Entity::Pln(Plain::make_plain(-0.3)),
		    Entity::Sph(Sphere::make_sphere(
		        origin - Vec3::make_vec3(0.0, 0.0, VIEWPORT_DEPTH),
		        0.3,
		    )),
        ];
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
            0.5 * self.do_trace(target_ray, depth - 1)
        }
    }

    // Here comes the most important function that actually do the tracing process of target ray.
    pub fn trace_ray_color(&self, target_ray: &Ray) -> Vec3 {
        self.do_trace(target_ray, ITERATION_DEPTH)
    }
}
