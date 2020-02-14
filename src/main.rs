use utilities::prelude::*;

use image;

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
        if first_image_pixel == second_image_pixel {
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
