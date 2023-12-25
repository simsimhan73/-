use std::{fs, io};
use std::io::{IoSliceMut, Read};
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use image::{GrayImage, image_dimensions, ImageBuffer, RgbImage};
use image::DynamicImage::ImageLuma8;

fn main() {
    println!("Hello, world!");

    let file_path = "icon_4.png";

    let img = image::open(Path::new(file_path)).unwrap();

    let gray = img.to_luma8();

    let (width, height) = gray.dimensions();

    let mut new_img = get_gradient_image(gray);

    let mut result = Vec::new();

    loop {
        if let Some(mut line) = new_img.pop() {
            result.append(&mut line)
        } else {
            break;
        }
    }



    let gradient_img = GrayImage::from_vec(width, height, result).unwrap();

    gradient_img.save("./out.png").unwrap();

    println!("{:?}", gradient_img);
}

fn get_gradient_image(input_array: GrayImage) -> Vec<Vec<u8>> {
    let mut gradient_image : Vec<Vec<u8>> = Vec::new();

    let height = input_array.height();
    let width = input_array.width();

    let mut i = 0;

    for y in 0..height {
        gradient_image.push(Vec::new());
        for x in 0..width {
            let mut x_gradient = 0;
            let mut y_gradient = 0;
            if y != 0 && y != height - 1 && x != 0 && x != width - 1 {
                x_gradient= get_gradient(input_array[(y, x - 1)].0[0], input_array[(y, x + 1)].0[0]);
                y_gradient = get_gradient(input_array[(y - 1, x)].0[0], input_array[(y + 1, x)].0[0]);
            }
                if x_gradient > 30 || y_gradient > 30 {
                gradient_image[y as usize].push(255);
            } else {
                gradient_image[y as usize].push(0);
            }
            i += 1;
        }
    }

    return gradient_image;
}

fn get_gradient(y1 : u8, y2 : u8) -> u8 {
    return if y2 > y1 {
        (y2 - y1) / 2
    } else {
        (y1 - y2) / 2
    }
}