use utilities::prelude::*;

use image;
use image::{ImageBuffer, Pixel, Rgba};

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

    let mut threshold = 0;

    if let Some(value) = arguments.get(3) {
        if let Ok(float) = value.parse::<f32>() {
            threshold = (std::u8::MAX as f32 * float) as u8;
        }
    }

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

    let mut difference_texture = ImageBuffer::from_pixel(width, height, Rgba([0, 0, 0, 0]));

    for ((first_image_pixel, second_image_pixel), difference_pixel) in first_image
        .pixels()
        .zip(second_image.pixels())
        .zip(difference_texture.pixels_mut())
    {
        let (first_r, first_g, first_b, _) = first_image_pixel.channels4();
        let (second_r, second_g, second_b, _) = second_image_pixel.channels4();

        let red_difference = channel_difference(first_r, second_r);
        let green_difference = channel_difference(first_g, second_g);
        let blue_difference = channel_difference(first_b, second_b);

        *difference_pixel = Rgba([red_difference, green_difference, blue_difference, 255]);

        if any_difference(red_difference, green_difference, blue_difference, threshold) {
            matching_pixel_count += 1;
        }
    }

    println!(
        "matching: {}, of: {}, percent: {}",
        matching_pixel_count,
        total_pixel_count,
        matching_pixel_count as f32 / total_pixel_count as f32
    );

    if let Err(err) = difference_texture.save("difference_texture.png") {
        println!("{:?}", err);
    }

    Ok(())
}

fn channel_difference(first: u8, second: u8) -> u8 {
    if second >= first {
        second - first
    } else {
        first - second
    }
}

fn any_difference(first: u8, second: u8, third: u8, threshold: u8) -> bool {
    for difference in [first, second, third].iter() {
        if *difference <= threshold {
            return true;
        }
    }

    false
}
