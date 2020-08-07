extern crate gl;
use gl::types::*;

use nalgebra_glm as glm;
use std::ffi::c_void;
use std::{mem, ptr};

pub fn cube_positions() -> [glm::TVec3<f32>; 10] {
    [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ]
}

fn vertices_pos_norm_tex() -> (usize, usize, usize, i32, [f32; (3 + 3 + 2) * 6 * 6]) {
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

pub fn create_textured_cube_with_normals_vao() -> u32 {
    let (npos, nnorm, ntex, stride, vertices) = vertices_pos_norm_tex();
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
        cube_vao
    }
}

pub fn create_textured_cube_with_normals_and_lamp_vaos() -> (u32, u32) {
    let (npos, nnorm, ntex, stride, vertices) = vertices_pos_norm_tex();
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

            // lamp vao
            gl::BindVertexArray(lamp_vao);
            gl::VertexAttribPointer(0, npos as i32, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        (cube_vao, lamp_vao)
    }
}

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
