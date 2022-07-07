// This is mod entity.
// Hittables are entities.

// Trait: Can_hit

use super::graphics::ray;

pub trait Can_hit{
	fn Get_hit_color (&self, target_ray:ray::Ray) -> Vec3{
		
	}
}

pub struct plain{		// A xOz plain served as a background.
	pub y:f64,
}

impl plain{
	pub fn make_plain(y:f64) -> plain {plain{y,}}
}
