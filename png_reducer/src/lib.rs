use image::{ImageBuffer, Rgba, Pixel};
use wasm_bindgen::prelude::*;
use std::io::Cursor;

#[wasm_bindgen]
pub fn reduce_png_colors(input: &[u8]) -> Vec<u8> {
    // Decode the PNG image
    let img = image::load_from_memory(input).unwrap();
    let img = img.to_rgba8();

    // Create a palette with the first 8 unique colors
    let mut palette = Vec::new();
    for pixel in img.pixels() {
        let color = pixel.to_rgba();
        if !palette.contains(&color) && palette.len() < 8 {
            palette.push(color);
        }
    }

    // Pad the palette with black if it has fewer than 8 colors
    while palette.len() < 8 {
        palette.push(Rgba([0, 0, 0, 255]));
    }

    // Map all pixels to the closest color in the palette
    let mut output_img = ImageBuffer::new(img.width(), img.height());
    for (x, y, pixel) in img.enumerate_pixels() {
        let color = pixel.to_rgba();
        let closest_color = find_closest_color(&color, &palette);
        output_img.put_pixel(x, y, closest_color);
    }

    // Encode the output image as a PNG
    let mut output_buffer = Cursor::new(Vec::new());
    output_img
        .write_to(&mut output_buffer, image::ImageFormat::Png)
        .unwrap();

    output_buffer.into_inner()
}

fn find_closest_color(color: &Rgba<u8>, palette: &[Rgba<u8>]) -> Rgba<u8> {
    palette
        .iter()
        .min_by_key(|&palette_color| color_distance(color, palette_color))
        .cloned()
        .unwrap()
}

fn color_distance(c1: &Rgba<u8>, c2: &Rgba<u8>) -> u32 {
    let r_diff = c1[0] as i32 - c2[0] as i32;
    let g_diff = c1[1] as i32 - c2[1] as i32;
    let b_diff = c1[2] as i32 - c2[2] as i32;
    (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as u32
}