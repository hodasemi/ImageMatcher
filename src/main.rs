use utilities::prelude::*;

use float_cmp::approx_eq;
use image;
use image::Pixel;

use std::env;

fn main() -> VerboseResult<()> {
    // first arguments is executable path
    let arguments: Vec<String> = env::args().collect();

    println!("arguments: {:?}", arguments);

    // load first image
    let first_image = match image::open(arguments.get(1).ok_or("missing path to first image")?) {
        Ok(image) => image.into_rgba(),
        Err(err) => create_error!(err.to_string()),
    };

    // load second image
    let second_image = match image::open(arguments.get(2).ok_or("missing path to second image")?) {
        Ok(image) => image.into_rgba(),
        Err(err) => create_error!(err.to_string()),
    };

    let threshold: Option<f32> = match arguments.get(3) {
        Some(value) => value.parse().ok(),
        None => None,
    };

    // make sure both images have the same width and height
    assert_eq!(
        first_image.dimensions(),
        second_image.dimensions(),
        "image dimensions don't match ({:?} != {:?})",
        first_image.dimensions(),
        second_image.dimensions()
    );

    let (width, height) = first_image.dimensions();

    let total_pixel_count = width * height;
    let mut matching_pixel_count = 0;

    for (first_image_pixel, second_image_pixel) in first_image.pixels().zip(second_image.pixels()) {
        let (first_r, first_g, first_b, first_a) = first_image_pixel.channels4();
        let (second_r, second_g, second_b, second_a) = second_image_pixel.channels4();

        if approx_eq!(
            f32,
            rgba_u8_to_f32(first_r),
            rgba_u8_to_f32(second_r),
            ulps = 2
        ) && approx_eq!(
            f32,
            rgba_u8_to_f32(first_g),
            rgba_u8_to_f32(second_g),
            ulps = 2
        ) && approx_eq!(
            f32,
            rgba_u8_to_f32(first_b),
            rgba_u8_to_f32(second_b),
            ulps = 2
        ) && approx_eq!(
            f32,
            rgba_u8_to_f32(first_a),
            rgba_u8_to_f32(second_a),
            ulps = 2
        ) {
            matching_pixel_count += 1;
        }
    }

    println!(
        "matching: {}, of: {}, percent: {}",
        matching_pixel_count,
        total_pixel_count,
        matching_pixel_count as f32 / total_pixel_count as f32
    );

    Ok(())
}

fn rgba_u8_to_f32(value: u8) -> f32 {
    value as f32 / 255.0
}
