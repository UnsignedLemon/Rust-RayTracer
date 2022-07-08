#![allow(unused_variables)]
#![allow(non_upper_case_globals)]

use std::{fs::File, process::exit};
use image::{ImageBuffer, RgbImage};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

extern crate lazy_static;

//---------------------------------    Modules    ------------------------------------------------------
pub mod graphics;
pub mod entity;
pub mod math_support;

use math_support::*;
use graphics::ray::Ray;
use entity::*;

//---------------------------------    Const Definations    --------------------------------------------	
const DEFAULT_COLOR:Vec3 = Vec3{x:1.0,y:0.0,z:0.5,};

const RATIO:f64 = 16.0 / 9.0;
const WIDTH:f64 = 800.0;
const HEIGHT:f64 = WIDTH / RATIO;

const VIEWPORT_HEIGHT:f64 = 2.0;
const VIEWPORT_WIDTH:f64 = RATIO * VIEWPORT_HEIGHT;
const VIEWPORT_DEPTH:f64 = 1.0;

const height:u32 = HEIGHT as u32;
const width:u32 = WIDTH as u32;
const quality:u8 = 60; // From 0 to 100


const origin:Vec3 = Vec3{x:0.0, y:0.0, z:0.0,};
const hor:Vec3 = Vec3{x:VIEWPORT_WIDTH, y:0.0, z:0.0,};
const ver:Vec3 = Vec3{x:0.0, y:VIEWPORT_HEIGHT, z:0.0,};

lazy_static::lazy_static!{
	static ref lower_left_corner:Vec3 = origin - hor/2.0 - ver/2.0 - Vec3{x:0.0, y:0.0, z:VIEWPORT_DEPTH};
	static ref background: entity::Plain = entity::Plain::make_plain(-1.0);
	static ref s1: entity::Sphere = entity::Sphere::make_sphere(origin - Vec3::make_vec3(0.0,0.0,VIEWPORT_DEPTH) , 0.3);
}

//-------------------------------------    Render function     ----------------------------------------
fn render_ray(u:f64, v:f64) -> Vec3{		
	// Trace ray with its origin in (0,0,0) and its direction pointing at far plain	coord (u,v);
	// Return color vector whose full value is 1.0 instead of 255.
	
	let target_ray:Ray = Ray::make_ray(
		origin ,
		*lower_left_corner + hor*u + ver*v - origin,
	);
	
	let bg_tm = background.get_hit_time(&target_ray);
	let sphere_tm = s1.get_hit_time(&target_ray);
	
	
	return match (bg_tm > 0.0, sphere_tm > 0.0){
		(false,false) => Vec3::make_vec3(1.0,1.0,1.0),
		(true,false) => background.get_hit_color(&target_ray),
		(false,true) => s1.get_hit_color(&target_ray),
		_ => {
			if sphere_tm < bg_tm {s1.get_hit_color(&target_ray)}
			else {background.get_hit_color(&target_ray)}
		},		// Always bg. However, this shows an ordinary process of render order checking.
	}
	
}

fn main() {
//----------------------------------------    Init    ------------------------------------------------------
	let path = "output/output.jpg";
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1
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

//------------------------------------    Render loop    -------------------------------------------
    for y in 0..height {
        for x in 0..width {
			// Calculate ray's pointing coord.
			// No anti-aliasing currently.
			let u:f64 = (x as f64) / (width as f64);
			let v:f64 = (y as f64) / (height as f64);
			
			// Do rendering with anti-aliasing.
			let dlt:f64 = 0.5 / (width as f64);
			let color_vec:Vec3 = 0.25 * (	render_ray(u,v) +
											render_ray(u+dlt, v) +
											render_ray(u, v+dlt) +
											render_ray(u+dlt,v+dlt));
		
			// Image generating.
			let pixel_color =[
				(color_vec.x* 255.0) .floor() as u8,
				(color_vec.y* 255.0) .floor() as u8,
				(color_vec.z* 255.0) .floor() as u8,	
			];	

			let pixel = img.get_pixel_mut(x, height - y - 1);
            *pixel = image::Rgb(pixel_color);
            progress.inc(1);
        }
    }
   
//----------------------------------    Never Mind    -------------------------------------------------
    
    progress.finish();

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
	}
	
	exit(0);
}
