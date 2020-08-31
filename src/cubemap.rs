use crate::util::img_info;
use image::GenericImageView;
use std::ffi::c_void;

pub struct Cubefaces {
    right: &'static str,
    left: &'static str,
    top: &'static str,
    bottom: &'static str,
    back: &'static str,
    front: &'static str,
}

impl
    From<(
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    )> for Cubefaces
{
    fn from(
        tuple: (
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
            &'static str,
        ),
    ) -> Self {
        Cubefaces {
            right: tuple.0,
            left: tuple.1,
            top: tuple.2,
            bottom: tuple.3,
            back: tuple.4,
            front: tuple.5,
        }
    }
}

impl From<Cubefaces> for [&'static str; 6] {
    fn from(faces: Cubefaces) -> Self {
        [
            /* +X */ faces.right,
            /* -X */ faces.left,
            /* +Y */ faces.top,
            /* -Y */ faces.bottom,
            /* +Z */ faces.back,
            /* -Z */ faces.front,
        ]
    }
}

pub unsafe fn load_cubemap<Faces>(faces: Faces) -> u32
where
    Faces: Into<Cubefaces>,
{
    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);

    let iterable: [&'static str; 6] = faces.into().into();
    for (i, face) in iterable.iter().enumerate() {
        let img = image::open(face).expect("Failed to load texture image");

        let texture_data = img.to_bytes();
        let (format, ..) = img_info(&img);
        gl::TexImage2D(
            gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
            0,
            format as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            format,
            gl::UNSIGNED_BYTE,
            &texture_data[0] as *const u8 as *const c_void,
        );
    }

    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MAG_FILTER,
        gl::LINEAR as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_S,
        gl::CLAMP_TO_EDGE as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_T,
        gl::CLAMP_TO_EDGE as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_WRAP_R,
        gl::CLAMP_TO_EDGE as i32,
    );

    texture_id
}
