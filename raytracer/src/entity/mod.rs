// This is mod entity.
// Hittables are entities.
#![allow(unused_variables)]

use crate::graphics::ray;
use crate::math_support::*;
use ray::Ray;
use crate::DEFAULT_COLOR;

// Trait CanHit
pub trait CanHit{
	fn get_hit_time (&self, target_ray: &ray::Ray) -> f64{
		-1.0 		// Not hit
	}
	
	fn get_hit_color(&self, target_ray: &Ray) -> Vec3{
		DEFAULT_COLOR		// Default color: color missing.
	}
	
	
}

//------------------------    Struct Plain    ------------------------------------------
pub struct Plain{		// A xOz plain served as a background.
	pub y:f64,
}

impl Plain{
	pub fn make_plain(y:f64) -> Plain {Plain{y,}}
}

impl CanHit for Plain{
	fn get_hit_time (&self, target_ray: &ray::Ray) -> f64{
		let product:f64 = dot(target_ray.get_dir(),Vec3{x:0.0, y:-1.0, z:0.0,});
		if product < 0.0 {
			return -1.0;
		}
		else{
			let y_spd = -target_ray.get_dir().y;
			return (target_ray.get_pos().y - self.y) / y_spd;
		}
	}
	
	fn get_hit_color(&self, target_ray: &Ray) -> Vec3{
		return Vec3::make_vec3(0.7,0.7,0.9);
	}
}

//-------------------------    Struct Sphere    ----------------------------------------

pub struct Sphere{
	centre:Vec3,
	r:f64,
}

impl Sphere{
	pub fn make_sphere(centre:Vec3, r:f64) -> Sphere{
		Sphere{
			centre,
			r,
		}
	}
	pub fn get_centre(&self) -> Vec3{return self.centre;}
	pub fn get_radius(&self) -> f64{return self.r;}
}

impl CanHit for Sphere{
	fn get_hit_time(&self, target_ray: &Ray) -> f64{
		let oc:Vec3 = target_ray.get_pos() - self.centre;

    	let b = dot(oc,target_ray.get_dir());
    	let c = dot(oc,oc) - self.r * self.r;

    	
  		let delta = b*b - c;
    	if delta < 0.0 {
    	    return -1.0; 	// Not hit.
    	} else {
    		return -b - delta.sqrt();		// First hit time.
   	 	}
	}
	
	fn get_hit_color(&self, target_ray:&Ray) -> Vec3{
		let tm:f64 = self.get_hit_time(&target_ray);
		let hit_pos:Vec3 = target_ray.get_pos() + tm*target_ray.get_dir();
		let hit_pos = hit_pos - self.centre;
		let hit_pos_normal = hit_pos.normalize();
		return hit_pos_normal;		// As the color of the surface.
	}
}
