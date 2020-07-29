// https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/2.2.hello_triangle_indexed/hello_triangle_indexed.cpp
mod chapter;
extern crate glfw;

use glfw::Context;
use std::ffi::{c_void, CString};
use std::{mem, ptr};

extern crate gl;
use gl::types::*;

const VERTEX_SHADER_SOURCE: &[u8] = include_bytes!("03_hello_triangle.vert");
const FRAGMENT_SHADER_SOURCE: &[u8] = include_bytes!("03_hello_triangle.frag");

pub fn main() {
    let (mut ctx, mut window, events) = chapter::init_window();

    let (shader_program, vao) = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(VERTEX_SHADER_SOURCE).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);
        chapter::check_for_errors(vertex_shader, gl::COMPILE_STATUS);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);
        chapter::check_for_errors(fragment_shader, gl::COMPILE_STATUS);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        chapter::check_for_errors(shader_program, gl::LINK_STATUS);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        #[rustfmt::skip]
        let vertices: [f32;12] = [
            0.5,  0.5, 0.0, // top right
            0.5, -0.5, 0.0, // bottom right
           -0.5, -0.5, 0.0, // bottom left
           -0.5,  0.5, 0.0, // top left
        ];
        #[rustfmt::skip]
        let indices: [u32;6] = [
            0, 1, 3,
            1, 2, 3
        ];

        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        {
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                &indices[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        (shader_program, vao)
    };

    while !window.should_close() {
        chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
