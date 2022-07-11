//----------------------------------    Struct Ray    ------------------------------------------------------------
use crate::math_support;

pub use math_support::Vec3;

#[derive(Debug,Copy,Clone)]
pub struct Ray{ 
	pub pos:Vec3,
	pub dir:Vec3,
}

impl Ray{
	pub fn make_ray(pos:Vec3, dir:Vec3) -> Ray {
		Ray{pos,dir:dir.normalize(),}
	}
	pub fn get_pos(&self) -> Vec3 {self.pos}
	pub fn get_dir(&self) -> Vec3 {self.dir}
	
}
