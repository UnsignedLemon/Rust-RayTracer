use std::{fs::File, process::exit};
use image::{ImageBuffer, RgbImage};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

pub mod graphics;
pub mod entity;
pub mod math_support;

use math_support::*;

use graphics::*;
use graphics::ray::Ray;

use entity::*;

// Const Definations

const DEFAULT_COLOR:Vec3 = Vec3{x:1.0,y:0.0,z:0.5,};

const RATIO:f64 = 16.0 / 9.0;
const WIDTH:f64 = 800.0;
const HEIGHT:f64 = WIDTH / RATIO;

const VIEWPORT_HEIGHT:f64 = 2.0;
const VIEWPORT_WIDTH:f64 = RATIO * VIEWPORT_HEIGHT;
const VIEWPORT_DEPTH:f64 = 1.0;

fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1
//-----------------------------------------------------------------------------------------------------
    let height = HEIGHT as u32;
    let width = WIDTH as u32;
    let quality = 60; // From 0 to 100
    let path = "output/output.jpg";
//-----------------------------------------------------------------------------------------------------

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(width.to_string() + &"x".to_string() + &height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width, height);
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));
//-------------------------------    My code starts here    --------------------------------------------

	let origin:Vec3 = Vec3::make_vec3(0.0,0.0,0.0);
	let hor:Vec3 = Vec3::make_vec3(VIEWPORT_WIDTH,0.0,0.0);
	let ver:Vec3 = Vec3::make_vec3(0.0,VIEWPORT_HEIGHT,0.0);
	
	let lower_left_corner = origin - hor/2.0 - ver/2.0 - Vec3::make_vec3(0.0, 0.0, VIEWPORT_DEPTH);

//--------------------------------    Objects to be Rendered    ---------------------------------------

	let background: entity::Plain = entity::Plain::make_plain(-1.0); 
	let s1: entity::Sphere = entity::Sphere::make_sphere(origin - Vec3::make_vec3(0.0,0.0,VIEWPORT_DEPTH) , 0.3);
	
//-----------------------------------------------------------------------------------------------------

    // Generate image
    for y in 0..height {
        for x in 0..width {
//---------------------------------    Process    ------------------------------------    
	
//			let pixel_color =[			// 3-u8 of RGB.
//                (y as f32 / height as f32 * 255.).floor() as u8,
//                ((x + height - y) as f32 / (height + width) as f32 * 255.).floor() as u8,
//                (x as f32 / height as f32 * 255.).floor() as u8,
//            ];
  
		let u:f64 = (x as f64) / ((width - 1) as f64);
		let v:f64 = (y as f64) / ((height - 1) as f64);

		let target_ray:Ray = Ray::make_ray(
				origin ,
				lower_left_corner +	hor*u + ver*v - origin,
				);

		let bg_tm = background.get_hit_time(&target_ray);
		let sphere_tm = s1.get_hit_time(&target_ray);
		
		
		let color_vec = match (bg_tm > 0.0, sphere_tm > 0.0){
			(false,false) => Vec3::make_vec3(1.0,1.0,1.0),
			(true,false) => background.get_hit_color(&target_ray),
			(false,true) => s1.get_hit_color(&target_ray),
			_ => {
				if sphere_tm < bg_tm {s1.get_hit_color(&target_ray)}
				else {background.get_hit_color(&target_ray)}
			},		// Always bg. However, this shows an ordinary process of render order checking.
		};
		
		let pixel_color =[
			(color_vec.x* 255.0) .floor() as u8,
			(color_vec.y* 255.0) .floor() as u8,
			(color_vec.z* 255.0) .floor() as u8,
		];	
//------------------------------------------------------------------------------------
	
			let pixel = img.get_pixel_mut(x, height - y - 1);
            *pixel = image::Rgb(pixel_color);
            progress.inc(1);
        }
    }
    progress.finish();
 

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
