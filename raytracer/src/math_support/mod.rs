// Math support of ray tracer.

//---------------------------    Module math_support    ----------------------------------------------------------

use std::ops;

#[derive(Debug,Clone,Copy)]
pub struct Vec3{		// Not using template. It's f32.
	pub x:f64,
	pub y:f64,
	pub z:f64,
}

impl ops::Add<Vec3> for Vec3{
	type Output=Vec3;

	fn add(self, other:Vec3) -> Vec3{
		Vec3{
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl ops::Sub<Vec3> for Vec3{
	type Output = Vec3;

	fn sub(self, other:Vec3) -> Vec3{
		Vec3{
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl ops::Mul<f64> for Vec3{
	type Output = Vec3;

	fn mul(self, val:f64) -> Vec3{
		Vec3{
			x: self.x * val,
			y: self.y * val,
			z: self.z * val,
		}
	}
}

impl ops::Div<f64> for Vec3{
	type Output = Vec3;

	fn div(self, val:f64) -> Vec3{
		if val==0.0 {
			dbg!("Divied by zero");
			Vec3{x:0.0, y:0.0, z:0.0,}
		}
		else{
			Vec3{
				x: self.x / val,
				y: self.y / val,
				z: self.z / val,
			}
		}
	}
}

impl Vec3{
	pub fn make_vec3(x:f64, y:f64, z:f64) -> Vec3{		// Construct Function of Vec3
		Vec3{
			x,
			y,
			z,
		}
	}
	
	pub fn get_len(self) -> f64{
		let sq_len:f64 = self.x * self.x + self.y * self.y + self.z * self.z;
		return sq_len.sqrt();	
	}
	
	pub fn normalize(self) -> Vec3{
		let len:f64 = self.get_len();
		return self / len;
	}
}
