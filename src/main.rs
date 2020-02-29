use utilities::prelude::*;

use image;
use image::{ImageBuffer, Pixel, Rgba};

use std::env;

fn main() -> VerboseResult<()> {
    // first arguments is executable path
    let arguments: Vec<String> = env::args().collect();

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

    let mut original_value = 0.0;
    let mut threshold = 0;

    if let Some(value) = arguments.get(3) {
        if let Ok(float) = value.parse::<f32>() {
            original_value = float;
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
    let mut perceptual_difference = 0.0;

    let mut difference_texture = ImageBuffer::from_pixel(width, height, Rgba([0, 0, 0, 0]));

    for ((first_image_pixel, second_image_pixel), difference_pixel) in first_image
        .pixels()
        .zip(second_image.pixels())
        .zip(difference_texture.pixels_mut())
    {
        let (first_r, first_g, first_b, _) = first_image_pixel.channels4();
        let (second_r, second_g, second_b, _) = second_image_pixel.channels4();

        let red_difference = channel_difference(first_r, second_r, threshold);
        let green_difference = channel_difference(first_g, second_g, threshold);
        let blue_difference = channel_difference(first_b, second_b, threshold);

        *difference_pixel = Rgba([red_difference, green_difference, blue_difference, 255]);

        perceptual_difference += normalize_color(color_difference(
            first_r, first_g, first_b, second_r, second_g, second_b, threshold,
        ));

        if !any_difference(red_difference, green_difference, blue_difference) {
            matching_pixel_count += 1;
        }
    }

    println!(
        "matching: {}, of: {}, percent: {}",
        matching_pixel_count,
        total_pixel_count,
        matching_pixel_count as f32 / total_pixel_count as f32
    );
    println!(
        "average perceptual difference: {}",
        perceptual_difference / total_pixel_count as f32
    );

    if let Err(err) =
        difference_texture.save(&format!("difference_texture_{:.2?}.png", original_value))
    {
        println!("{:?}", err);
    }

    Ok(())
}

/// https://en.wikipedia.org/wiki/Color_difference#Euclidean
fn color_difference(
    first_r: u8,
    first_g: u8,
    first_b: u8,
    second_r: u8,
    second_g: u8,
    second_b: u8,
    threshold: u8,
) -> u8 {
    let r_mean = first_r as f32 + second_r as f32 / 2.0;

    let delta_r = first_r as f32 - second_r as f32;
    let delta_g = first_g as f32 - second_g as f32;
    let delta_b = first_b as f32 - second_b as f32;

    let diff = ((2.0 + (r_mean / 256.0)) * (delta_r * delta_r)
        + 4.0 * (delta_g * delta_g)
        + (2.0 + ((255.0 - r_mean) / 256.0)) * (delta_b * delta_b))
        .sqrt()
        .abs() as u8;

    if diff > threshold {
        diff
    } else {
        0
    }
}

fn normalize_color(c: u8) -> f32 {
    c as f32 / 255.0
}

fn channel_difference(first: u8, second: u8, threshold: u8) -> u8 {
    let difference = if second >= first {
        second - first
    } else {
        first - second
    };

    if difference <= threshold {
        0
    } else {
        difference
    }
}

fn any_difference(first: u8, second: u8, third: u8) -> bool {
    for difference in [first, second, third].iter() {
        if *difference != 0 {
            return true;
        }
    }

    false
}
