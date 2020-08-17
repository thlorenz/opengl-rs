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

pub fn create_textured_plane_vao() -> u32 {
    let (npos, ntex, stride, vertices) = vertices_plane_pos_tex();
    create_textured_vao(npos, ntex, stride, vertices.to_vec())
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

pub fn create_textured_transparent_vao() -> u32 {
    let (npos, ntex, stride, vertices) = vertices_transparent_pos_tex();
    create_textured_vao(npos, ntex, stride, vertices.to_vec())
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
