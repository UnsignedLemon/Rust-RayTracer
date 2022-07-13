#![allow(unused_variables)]
#![allow(non_upper_case_globals)]

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::File, process::exit};

extern crate lazy_static;

//---------------------------------    Modules    ----------------------------------------
pub mod entity;
pub mod graphics;
pub mod math_support;
pub mod world;

use graphics::Camera;
use math_support::*;
use world::World;

//---------------------------------    Const Definations    ------------------------------

//---------------------------------    Camera & Picture    -------------------------------
const RATIO: f64 = 16.0 / 9.0;
const WIDTH: f64 = 1200.0;
const HEIGHT: f64 = WIDTH / RATIO;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = RATIO * VIEWPORT_HEIGHT;

const height: u32 = HEIGHT as u32;
const width: u32 = WIDTH as u32;
const quality: u8 = 60; // From 0 to 100

const flash_t0: f64 = 0.0;
const flash_t1: f64 = 1.0;

//--------------------------------    Coordinate    --------------------------------------
const origin: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

//--------------------------------    World Settings    ----------------------------------
lazy_static::lazy_static! {

    static ref wld:World = World::make_world();
    static ref cmr:Camera = Camera::make_camera(Vec3::make_vec3(0.0,0.0,0.0),
                                                Vec3::make_vec3(-1.9,0.2,1.9),
                                                std::f64::consts::PI / 6.0,
                                                0.005, flash_t0, flash_t1,
    );
}

//--------------------------------     Render Parameters    ------------------------------
const ITERATION_DEPTH: i32 = 100;
const SAMPLES_PER_PIXEL: i32 = 1000;

fn main() {
    //----------------------------------------    Init    --------------------------------
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

    //------------------------------------    Render loop    -----------------------------
    for y in 0..height {
        for x in 0..width {
            // Do render with anti-aliasing and gamma-correction.
            let color_vec: Vec3 = graphics::render_pixel(x, y);

            // Image generating.
            let pixel_color = [
                (color_vec.x * 255.0).floor() as u8,
                (color_vec.y * 255.0).floor() as u8,
                (color_vec.z * 255.0).floor() as u8,
            ];

            let pixel = img.get_pixel_mut(x, height - y - 1);
            *pixel = image::Rgb(pixel_color);
            progress.inc(1);
        }
    }

    //----------------------------------    Never Mind    --------------------------------

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
