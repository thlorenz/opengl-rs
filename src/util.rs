extern crate gl;
use image::{DynamicImage, GenericImageView};
use std::ffi::c_void;

pub fn load_texture(path: &str, vflip: bool) -> u32 {
    let mut texture = 0;

    let img = image::open(path).expect("Failed to load texture image");
    let format = match img {
        DynamicImage::ImageLuma8(_) | DynamicImage::ImageLuma16(_) => gl::RED,
        DynamicImage::ImageLumaA8(_) | DynamicImage::ImageLumaA16(_) => gl::RG,
        DynamicImage::ImageRgb8(_) | DynamicImage::ImageRgb16(_) => gl::RGB,
        DynamicImage::ImageRgba8(_) | DynamicImage::ImageRgba16(_) => gl::RGBA,
        DynamicImage::ImageBgr8(_) | DynamicImage::ImageBgra8(_) => gl::BGR,
    };

    let img = if vflip { img.flipv() } else { img };

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        let texture_data = img.to_bytes();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            format,
            gl::UNSIGNED_BYTE,
            &texture_data[0] as *const u8 as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // Wrapping
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        // Filtering
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }
    texture
}
