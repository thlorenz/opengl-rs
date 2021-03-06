extern crate gl;

use gl::types::*;
use nalgebra_glm as glm;
use std::ffi::c_void;
use std::mem;
use std::ptr;

pub fn create_textured_cube_vao() -> u32 {
    let (npos, ntex, stride, vertices) = vertices_box_pos_tex();
    create_textured_vao(npos, ntex, stride, vertices.to_vec())
}

pub fn create_normals_textured_cube_vao() -> u32 {
    let (npos, nnorm, ntex, stride, vertices) = vertices_box_pos_norm_tex();
    create_normals_textured_vao(npos, nnorm, ntex, stride, vertices.to_vec())
}

pub fn create_textured_plane_vao() -> u32 {
    let (npos, ntex, stride, vertices) = vertices_plane_pos_tex();
    create_textured_vao(npos, ntex, stride, vertices.to_vec())
}

pub fn create_textured_quad_vao() -> u32 {
    let (npos, ntex, stride, vertices) = vertices_quad_pos_tex();
    create_textured_vao(npos, ntex, stride, vertices.to_vec())
}

pub fn create_skybox_vao() -> u32 {
    let (npos, stride, vertices) = vertices_skybox_pos();
    create_vao(npos, stride, vertices.to_vec())
}

pub fn create_textured_transparent_vao() -> u32 {
    let (npos, ntex, stride, vertices) = vertices_transparent_pos_tex();
    create_textured_vao(npos, ntex, stride, vertices.to_vec())
}

pub unsafe fn setup_texture_framebuffer(screen_width: u32, screen_height: u32) -> (u32, u32) {
    // Setup framebuffer
    let mut framebuffer = 0;
    let mut texture_color_buffer = 0;
    gl::GenFramebuffers(1, &mut framebuffer);
    gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
    {
        // Create color attachment texture
        gl::GenTextures(1, &mut texture_color_buffer);
        gl::BindTexture(gl::TEXTURE_2D, texture_color_buffer);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            screen_width as i32,
            screen_height as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            texture_color_buffer,
            0,
        );

        // Create RenderBuffer object for depth and stencil attachement
        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            screen_width as i32,
            screen_height as i32,
        );
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        );

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            eprintln!("ERROR:FRAMEBUFFER: Framebuffer not completed");
        }
    }
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    (framebuffer, texture_color_buffer)
}

#[rustfmt::skip]
pub fn vec3_transparent_pos() -> [glm::Vec3; 5] {
    [
        glm::vec3(-1.5, 0.0, -0.48),
        glm::vec3( 1.5, 0.0,  0.51),
        glm::vec3( 0.0, 0.0,  0.7),
        glm::vec3(-0.3, 0.0, -2.3),
        glm::vec3( 0.5, 0.0, -0.6),
    ]
}

fn create_normals_textured_vao(
    npos: usize,
    nnorm: usize,
    ntex: usize,
    stride: i32,
    vertices: Vec<f32>,
) -> u32 {
    unsafe {
        let (mut vbo, mut vao) = (0, 0);
        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut vao);
        {
            // load vertices into vbo
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            // vao
            gl::BindVertexArray(vao);
            gl::VertexAttribPointer(0, npos as i32, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            // normal attrib
            gl::VertexAttribPointer(
                1,
                nnorm as i32,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (npos as i32 * mem::size_of::<GLfloat>() as GLsizei) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            // texture attrib
            gl::VertexAttribPointer(
                2,
                ntex as i32,
                gl::FLOAT,
                gl::FALSE,
                stride,
                ((npos + nnorm) as i32 * mem::size_of::<GLfloat>() as GLsizei) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);
        }
        vao
    }
}

fn create_textured_vao(npos: usize, ntex: usize, stride: i32, vertices: Vec<f32>) -> u32 {
    unsafe {
        let (mut vbo, mut cube_vao) = (0, 0);
        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut cube_vao);
        {
            // load vertices into vbo
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            // vao
            gl::BindVertexArray(cube_vao);
            gl::VertexAttribPointer(0, npos as i32, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            // texture attrib
            gl::VertexAttribPointer(
                1,
                ntex as i32,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (npos as i32 * mem::size_of::<GLfloat>() as GLsizei) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }
        cube_vao
    }
}

fn create_vao(npos: usize, stride: i32, vertices: Vec<f32>) -> u32 {
    unsafe {
        let (mut vbo, mut vao) = (0, 0);
        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut vao);
        {
            // load vertices into vbo
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            // vao
            gl::BindVertexArray(vao);
            gl::VertexAttribPointer(0, npos as i32, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
        }
        vao
    }
}

fn vertices_skybox_pos() -> (usize, i32, [f32; 3 * 6 * 6]) {
    const NPOS: usize = 3;
    const NROWS: usize = 6;
    const NSIDES: usize = 6;
    let stride = NPOS as i32 * mem::size_of::<GLfloat>() as GLsizei;
    #[rustfmt::skip]
    let vertices: [f32;  NPOS * NROWS * NSIDES] = [
       -1.0,  1.0, -1.0,
       -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0,  1.0, -1.0,
       -1.0,  1.0, -1.0,

       -1.0, -1.0,  1.0,
       -1.0, -1.0, -1.0,
       -1.0,  1.0, -1.0,
       -1.0,  1.0, -1.0,
       -1.0,  1.0,  1.0,
       -1.0, -1.0,  1.0,

        1.0, -1.0, -1.0,
        1.0, -1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0, -1.0,
        1.0, -1.0, -1.0,

       -1.0, -1.0,  1.0,
       -1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0, -1.0,  1.0,
       -1.0, -1.0,  1.0,

       -1.0,  1.0, -1.0,
        1.0,  1.0, -1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,
       -1.0,  1.0,  1.0,
       -1.0,  1.0, -1.0,

       -1.0, -1.0, -1.0,
       -1.0, -1.0,  1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
       -1.0, -1.0,  1.0,
        1.0, -1.0,  1.0
    ];
    (NPOS, stride, vertices)
}

fn vertices_transparent_pos_tex() -> (usize, usize, i32, [f32; (3 + 2) * 1 * 6]) {
    const NPOS: usize = 3;
    const NTEX: usize = 2;
    const NROWS: usize = 1;
    const NSIDES: usize = 6;
    let stride = (NPOS + NTEX) as i32 * mem::size_of::<GLfloat>() as GLsizei;

    #[rustfmt::skip]
    let vertices: [f32;  (NPOS + NTEX) * NROWS * NSIDES] = [
        //   position           texture
         0.0,  0.5,  0.0,      0.0,  1.0,
         0.0, -0.5,  0.0,      0.0,  0.0,
         1.0, -0.5,  0.0,      1.0,  0.0,

         0.0,  0.5,  0.0,      0.0,  1.0,
         1.0, -0.5,  0.0,      1.0,  0.0,
         1.0,  0.5,  0.0,      1.0,  1.0
    ];
    (NPOS, NTEX, stride, vertices)
}

fn vertices_plane_pos_tex() -> (usize, usize, i32, [f32; (3 + 2) * 1 * 6]) {
    const NPOS: usize = 3;
    const NTEX: usize = 2;
    const NROWS: usize = 1;
    const NSIDES: usize = 6;
    let stride = (NPOS + NTEX) as i32 * mem::size_of::<GLfloat>() as GLsizei;

    #[rustfmt::skip]
    let vertices: [f32;  (NPOS + NTEX) * NROWS * NSIDES] = [
        //   position           texture
         5.0, -0.5,  5.0,      2.0, 0.0,
        -5.0, -0.5,  5.0,      0.0, 0.0,
        -5.0, -0.5, -5.0,      0.0, 2.0,

         5.0, -0.5,  5.0,      2.0, 0.0,
        -5.0, -0.5, -5.0,      0.0, 2.0,
         5.0, -0.5, -5.0,      2.0, 2.0
    ];
    (NPOS, NTEX, stride, vertices)
}

fn vertices_quad_pos_tex() -> (usize, usize, i32, [f32; (2 + 2) * 1 * 6]) {
    const NPOS: usize = 2;
    const NTEX: usize = 2;
    const NROWS: usize = 1;
    const NSIDES: usize = 6;
    let stride = (NPOS + NTEX) as i32 * mem::size_of::<GLfloat>() as GLsizei;

    #[rustfmt::skip]
    let vertices: [f32;  (NPOS + NTEX) * NROWS * NSIDES] = [
        //   position    texture
        -1.0,  1.0,     0.0, 1.0,
        -1.0, -1.0,     0.0, 0.0,
         1.0, -1.0,     1.0, 0.0,

        -1.0,  1.0,     0.0, 1.0,
         1.0, -1.0,     1.0, 0.0,
         1.0,  1.0,     1.0, 1.0
    ];
    (NPOS, NTEX, stride, vertices)
}

fn vertices_box_pos_tex() -> (usize, usize, i32, [f32; (3 + 2) * 6 * 6]) {
    const NPOS: usize = 3;
    const NTEX: usize = 2;
    const NROWS: usize = 6;
    const NSIDES: usize = 6;
    let stride = (NPOS + NTEX) as i32 * mem::size_of::<GLfloat>() as GLsizei;
    #[rustfmt::skip]
    let vertices: [f32;  (NPOS + NTEX) * NROWS * NSIDES] = [
        //   position           texture
        -0.5, -0.5, -0.5,      0.0,  0.0,
         0.5, -0.5, -0.5,      1.0,  0.0,
         0.5,  0.5, -0.5,      1.0,  1.0,
         0.5,  0.5, -0.5,      1.0,  1.0,
        -0.5,  0.5, -0.5,      0.0,  1.0,
        -0.5, -0.5, -0.5,      0.0,  0.0,

        -0.5, -0.5,  0.5,      0.0,  0.0,
         0.5, -0.5,  0.5,      1.0,  0.0,
         0.5,  0.5,  0.5,      1.0,  1.0,
         0.5,  0.5,  0.5,      1.0,  1.0,
        -0.5,  0.5,  0.5,      0.0,  1.0,
        -0.5, -0.5,  0.5,      0.0,  0.0,

        -0.5,  0.5,  0.5,      1.0,  0.0,
        -0.5,  0.5, -0.5,      1.0,  1.0,
        -0.5, -0.5, -0.5,      0.0,  1.0,
        -0.5, -0.5, -0.5,      0.0,  1.0,
        -0.5, -0.5,  0.5,      0.0,  0.0,
        -0.5,  0.5,  0.5,      1.0,  0.0,

         0.5,  0.5,  0.5,      1.0,  0.0,
         0.5,  0.5, -0.5,      1.0,  1.0,
         0.5, -0.5, -0.5,      0.0,  1.0,
         0.5, -0.5, -0.5,      0.0,  1.0,
         0.5, -0.5,  0.5,      0.0,  0.0,
         0.5,  0.5,  0.5,      1.0,  0.0,

        -0.5, -0.5, -0.5,      0.0,  1.0,
         0.5, -0.5, -0.5,      1.0,  1.0,
         0.5, -0.5,  0.5,      1.0,  0.0,
         0.5, -0.5,  0.5,      1.0,  0.0,
        -0.5, -0.5,  0.5,      0.0,  0.0,
        -0.5, -0.5, -0.5,      0.0,  1.0,

        -0.5,  0.5, -0.5,      0.0,  1.0,
         0.5,  0.5, -0.5,      1.0,  1.0,
         0.5,  0.5,  0.5,      1.0,  0.0,
         0.5,  0.5,  0.5,      1.0,  0.0,
        -0.5,  0.5,  0.5,      0.0,  0.0,
        -0.5,  0.5, -0.5,      0.0,  1.0
    ];
    (NPOS, NTEX, stride, vertices)
}

fn vertices_box_pos_norm_tex() -> (usize, usize, usize, i32, [f32; (3 + 3 + 2) * 6 * 6]) {
    const NPOS: usize = 3;
    const NNORM: usize = 3;
    const NTEX: usize = 2;
    const NROWS: usize = 6;
    const NSIDES: usize = 6;
    let stride = (NPOS + NNORM + NTEX) as i32 * mem::size_of::<GLfloat>() as GLsizei;
    #[rustfmt::skip]
    let vertices: [f32;  (NPOS + NNORM + NTEX) * NROWS * NSIDES] = [
        //   position               normal            texture
        -0.5, -0.5, -0.5,      0.0,  0.0, -1.0,      0.0,  0.0,
         0.5, -0.5, -0.5,      0.0,  0.0, -1.0,      1.0,  0.0,
         0.5,  0.5, -0.5,      0.0,  0.0, -1.0,      1.0,  1.0,
         0.5,  0.5, -0.5,      0.0,  0.0, -1.0,      1.0,  1.0,
        -0.5,  0.5, -0.5,      0.0,  0.0, -1.0,      0.0,  1.0,
        -0.5, -0.5, -0.5,      0.0,  0.0, -1.0,      0.0,  0.0,

        -0.5, -0.5,  0.5,      0.0,  0.0,  1.0,      0.0,  0.0,
         0.5, -0.5,  0.5,      0.0,  0.0,  1.0,      1.0,  0.0,
         0.5,  0.5,  0.5,      0.0,  0.0,  1.0,      1.0,  1.0,
         0.5,  0.5,  0.5,      0.0,  0.0,  1.0,      1.0,  1.0,
        -0.5,  0.5,  0.5,      0.0,  0.0,  1.0,      0.0,  1.0,
        -0.5, -0.5,  0.5,      0.0,  0.0,  1.0,      0.0,  0.0,

        -0.5,  0.5,  0.5,     -1.0,  0.0,  0.0,      1.0,  0.0,
        -0.5,  0.5, -0.5,     -1.0,  0.0,  0.0,      1.0,  1.0,
        -0.5, -0.5, -0.5,     -1.0,  0.0,  0.0,      0.0,  1.0,
        -0.5, -0.5, -0.5,     -1.0,  0.0,  0.0,      0.0,  1.0,
        -0.5, -0.5,  0.5,     -1.0,  0.0,  0.0,      0.0,  0.0,
        -0.5,  0.5,  0.5,     -1.0,  0.0,  0.0,      1.0,  0.0,

         0.5,  0.5,  0.5,      1.0,  0.0,  0.0,      1.0,  0.0,
         0.5,  0.5, -0.5,      1.0,  0.0,  0.0,      1.0,  1.0,
         0.5, -0.5, -0.5,      1.0,  0.0,  0.0,      0.0,  1.0,
         0.5, -0.5, -0.5,      1.0,  0.0,  0.0,      0.0,  1.0,
         0.5, -0.5,  0.5,      1.0,  0.0,  0.0,      0.0,  0.0,
         0.5,  0.5,  0.5,      1.0,  0.0,  0.0,      1.0,  0.0,

        -0.5, -0.5, -0.5,      0.0, -1.0,  0.0,      0.0,  1.0,
         0.5, -0.5, -0.5,      0.0, -1.0,  0.0,      1.0,  1.0,
         0.5, -0.5,  0.5,      0.0, -1.0,  0.0,      1.0,  0.0,
         0.5, -0.5,  0.5,      0.0, -1.0,  0.0,      1.0,  0.0,
        -0.5, -0.5,  0.5,      0.0, -1.0,  0.0,      0.0,  0.0,
        -0.5, -0.5, -0.5,      0.0, -1.0,  0.0,      0.0,  1.0,

        -0.5,  0.5, -0.5,      0.0,  1.0,  0.0,      0.0,  1.0,
         0.5,  0.5, -0.5,      0.0,  1.0,  0.0,      1.0,  1.0,
         0.5,  0.5,  0.5,      0.0,  1.0,  0.0,      1.0,  0.0,
         0.5,  0.5,  0.5,      0.0,  1.0,  0.0,      1.0,  0.0,
        -0.5,  0.5,  0.5,      0.0,  1.0,  0.0,      0.0,  0.0,
        -0.5,  0.5, -0.5,      0.0,  1.0,  0.0,      0.0,  1.0
    ];
    (NPOS, NNORM, NTEX, stride, vertices)
}
