pub mod ray;

use ray::Ray;
use crate::*;
use crate::math_support::*;

pub fn render_pixel(x:u32, y:u32) -> Vec3{
	let mut samples = SAMPLES_PER_PIXEL;
	let mut color:Vec3 = Vec3::make_vec3(0.0,0.0,0.0);
	
	while samples > 0{		// Anti aliasing with SAMPLES_PER_PIXEL samples.
	
		let dlt_x = rand_0_1();	
		let dlt_y = rand_0_1();
		
		let u = (x as f64 + dlt_x) / WIDTH;
		let v = (y as f64 + dlt_y) / HEIGHT;
		
		let target_ray:Ray = Ray::make_ray(
			origin ,
			*lower_left_corner + hor*u + ver*v - origin,
		);
		
		color = color + wld.trace_ray_color(&target_ray);
		
		samples = samples - 1;
	}
	
	color = color / (SAMPLES_PER_PIXEL as f64);
	color = color.sqrt_for_gamma_correction();		// Gamma correction.
	return color;

}
