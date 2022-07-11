pub mod ray;

use crate::math_support::*;
use crate::*;
use ray::Ray;

//-----------------------    Render target pixel    --------------------------------------
pub fn render_pixel(x: u32, y: u32) -> Vec3 {
    let mut samples = SAMPLES_PER_PIXEL;
    let mut color: Vec3 = Vec3::make_vec3(0.0, 0.0, 0.0);

    while samples > 0 {
        // Anti aliasing with SAMPLES_PER_PIXEL samples.

        let dlt_x = rand_0_1();
        let dlt_y = rand_0_1();

        let u = (x as f64) + dlt_x;
        let v = (y as f64) + dlt_y;

        let target_ray: Ray = cmr.get_ray_of_pixel(u, v);

        color = color + wld.trace_ray_color(&target_ray);

        samples -= 1;
    }

    color = color / (SAMPLES_PER_PIXEL as f64);
    color = color.sqrt_for_gamma_correction(); // Gamma correction.
    color
}

//------------------------    Struct camera    -------------------------------------------
pub struct Camera {
    pub dir: Vec3,
    pub pos: Vec3,
    pub right: Vec3,
    pub up: Vec3,

    pub lower_left_corner: Vec3,
    pub viewport_depth: f64,
}

impl Camera {
    pub fn make_camera(lookat: Vec3, pos: Vec3, deg: f64) -> Camera {
        let dir = (lookat - pos).normalize();
        let world_up: Vec3 = Vec3::make_vec3(0.0, 1.0, 0.0);

        let right = cross(dir, world_up);
        let right = right.normalize() * VIEWPORT_WIDTH;
        let up = cross(right, dir);
        let up = up.normalize() * VIEWPORT_HEIGHT;

        let viewport_depth = 0.5 * VIEWPORT_HEIGHT / ((0.5 * deg).tan());
        let lower_left_corner = dir * viewport_depth - right * 0.5 - up * 0.5;

        Camera {
            dir,
            pos,
            right,
            up,
            lower_left_corner,
            viewport_depth,
        }
    }

    pub fn get_ray_of_pixel(&self, u: f64, v: f64) -> Ray {
        // u/v can be non-integer, which is used for multi-sampling.
        Ray::make_ray(
            self.pos,
            self.lower_left_corner + self.right * u / WIDTH + self.up * v / HEIGHT,
        )
    }
}
