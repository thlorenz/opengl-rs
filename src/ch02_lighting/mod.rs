extern crate gl;
use gl::types::*;

use std::ffi::c_void;
use std::{mem, ptr};

pub fn create_cube_with_normals_and_lamp_vaos() -> (u32, u32) {
    const NPOS: usize = 3;
    const NNORM: usize = 3;
    const NROWS: usize = 6;
    const NSIDES: usize = 6;
    let stride = (NPOS + NNORM) as i32 * mem::size_of::<GLfloat>() as GLsizei;
    #[rustfmt::skip]
    let vertices: [f32;  (NPOS + NNORM) * NROWS * NSIDES] = [
        //   position             normal
        -0.5, -0.5, -0.5,     0.0,  0.0, -1.0,
         0.5, -0.5, -0.5,     0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,     0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,     0.0,  0.0, -1.0,
        -0.5,  0.5, -0.5,     0.0,  0.0, -1.0,
        -0.5, -0.5, -0.5,     0.0,  0.0, -1.0,

        -0.5, -0.5,  0.5,     0.0,  0.0, 1.0,
         0.5, -0.5,  0.5,     0.0,  0.0, 1.0,
         0.5,  0.5,  0.5,     0.0,  0.0, 1.0,
         0.5,  0.5,  0.5,     0.0,  0.0, 1.0,
        -0.5,  0.5,  0.5,     0.0,  0.0, 1.0,
        -0.5, -0.5,  0.5,     0.0,  0.0, 1.0,

        -0.5,  0.5,  0.5,    -1.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,    -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,    -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,    -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5,    -1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5,    -1.0,  0.0,  0.0,

         0.5,  0.5,  0.5,     1.0,  0.0,  0.0,
         0.5,  0.5, -0.5,     1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,     1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,     1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,     1.0,  0.0,  0.0,
         0.5,  0.5,  0.5,     1.0,  0.0,  0.0,

        -0.5, -0.5, -0.5,     0.0, -1.0,  0.0,
         0.5, -0.5, -0.5,     0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,     0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,     0.0, -1.0,  0.0,
        -0.5, -0.5,  0.5,     0.0, -1.0,  0.0,
        -0.5, -0.5, -0.5,     0.0, -1.0,  0.0,

        -0.5,  0.5, -0.5,     0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,     0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,     0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,     0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,     0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5,     0.0,  1.0,  0.0
    ];

    unsafe {
        let (mut vbo, mut cube_vao, mut lamp_vao) = (0, 0, 0);
        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut cube_vao);
        gl::GenVertexArrays(1, &mut lamp_vao);
        {
            // load vertices into vbo
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            // cube vao
            gl::BindVertexArray(cube_vao);
            gl::VertexAttribPointer(0, NPOS as i32, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            // normal attrib
            gl::VertexAttribPointer(
                1,
                NNORM as i32,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (NPOS as i32 * mem::size_of::<GLfloat>() as GLsizei) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            // lamp vao
            gl::BindVertexArray(lamp_vao);
            gl::VertexAttribPointer(0, NPOS as i32, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        (cube_vao, lamp_vao)
    }
}
pub fn create_box_vao() -> u32 {
    const NPOS: usize = 3;
    const NROWS: usize = 6;
    const NSIDES: usize = 6;
    let stride = NPOS as i32 * mem::size_of::<GLfloat>() as GLsizei;
    #[rustfmt::skip]
    let vertices: [f32;  NPOS * NROWS * NSIDES] = [
       -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
       -0.5,  0.5, -0.5,
       -0.5, -0.5, -0.5,

       -0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
       -0.5,  0.5,  0.5,
       -0.5, -0.5,  0.5,

       -0.5,  0.5,  0.5,
       -0.5,  0.5, -0.5,
       -0.5, -0.5, -0.5,
       -0.5, -0.5, -0.5,
       -0.5, -0.5,  0.5,
       -0.5,  0.5,  0.5,

        0.5,  0.5,  0.5,
        0.5,  0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5,  0.5,
        0.5,  0.5,  0.5,

       -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
       -0.5, -0.5,  0.5,
       -0.5, -0.5, -0.5,

       -0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
       -0.5,  0.5,  0.5,
       -0.5,  0.5, -0.5,
    ];

    unsafe {
        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, NPOS as i32, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        vao
    }
}
