extern crate gl;
use gl::types::*;

use std::ffi::c_void;
use std::{mem, ptr};

pub fn create_box_with_normal_vao() -> u32 {
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

            // normal
            gl::VertexAttribPointer(
                1,
                NNORM as i32,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (NPOS as i32 * mem::size_of::<GLfloat>() as GLsizei) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        vao
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
