use image::{Rgba, Pixel};
use wasm_bindgen::prelude::*;
use std::io::Cursor;
use png::{Encoder, ColorType}; // Using `png` crate for indexed PNG support

#[wasm_bindgen]
pub fn reduce_png_colors(input: &[u8]) -> Vec<u8> {
    // Decode the PNG image
    let img = image::load_from_memory(input).unwrap().to_rgba8();
    let (width, height) = img.dimensions();

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

    // Create an indexed image where each pixel is mapped to a palette index
    let mut indexed_pixels = vec![0u8; (width * height) as usize];

    for (i, pixel) in img.pixels().enumerate() {
        let color = pixel.to_rgba();
        let closest_index = find_closest_color_index(&color, &palette);
        indexed_pixels[i] = closest_index as u8;
    }

    // Flatten palette into RGB triplets (excluding alpha)
    let palette_bytes: Vec<u8> = palette.iter()
        .flat_map(|c| vec![c[0], c[1], c[2]]) // Only store RGB
        .collect();

    // Encode the output image as an indexed PNG using `png` crate
    let mut output_buffer = Cursor::new(Vec::new());
    {
        let mut encoder = Encoder::new(&mut output_buffer, width, height);
        encoder.set_color(ColorType::Indexed);
        encoder.set_palette(palette_bytes);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&indexed_pixels).unwrap();
    }

    output_buffer.into_inner()
}

fn find_closest_color_index(color: &Rgba<u8>, palette: &[Rgba<u8>]) -> usize {
    palette.iter()
        .enumerate()
        .min_by_key(|(_, &palette_color)| color_distance(color, &palette_color))
        .map(|(index, _)| index)
        .unwrap()
}

fn color_distance(c1: &Rgba<u8>, c2: &Rgba<u8>) -> u32 {
    let r_diff = c1[0] as i32 - c2[0] as i32;
    let g_diff = c1[1] as i32 - c2[1] as i32;
    let b_diff = c1[2] as i32 - c2[2] as i32;
    (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as u32
}
