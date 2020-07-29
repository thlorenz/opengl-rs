// https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/2.1.hello_triangle/hello_triangle.cpp
mod chapter;
extern crate glfw;

use glfw::Context;
use std::ffi::{c_void, CString};
use std::str;
use std::{mem, ptr};

extern crate gl;
use gl::types::*;

const VERTEX_SHADER_SOURCE: &[u8] = include_bytes!("03_hello_triangle.vert");
const FRAGMENT_SHADER_SOURCE: &[u8] = include_bytes!("03_hello_triangle.frag");

fn check_for_errors(item: u32, status_type: u32) {
    let mut success = gl::FALSE as GLint;
    let mut info_log: Vec<u8> = Vec::with_capacity(512);
    unsafe {
        info_log.set_len(512 - 1); // skip \0 char
        if status_type == gl::COMPILE_STATUS {
            gl::GetShaderiv(item, status_type, &mut success);
        } else if status_type == gl::LINK_STATUS {
            gl::GetProgramiv(item, status_type, &mut success);
        }
        if success != gl::TRUE as GLint {
            if status_type == gl::COMPILE_STATUS {
                gl::GetShaderInfoLog(
                    item,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                eprintln!(
                    "Compilation failed\n{}",
                    str::from_utf8_unchecked(&info_log)
                );
            } else if status_type == gl::LINK_STATUS {
                gl::GetProgramInfoLog(
                    item,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                eprintln!("Linking failed\n{}", str::from_utf8_unchecked(&info_log));
            }
        }
    }
}

pub fn main() {
    let (mut ctx, mut window, events) = chapter::init_window();

    let (shader_program, vao) = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(VERTEX_SHADER_SOURCE).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);
        check_for_errors(vertex_shader, gl::COMPILE_STATUS);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);
        check_for_errors(fragment_shader, gl::COMPILE_STATUS);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        check_for_errors(shader_program, gl::LINK_STATUS);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        #[rustfmt::skip]
        let vertices: [f32;9] = [
           -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0,  0.5, 0.0
        ];

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

        (shader_program, vao)
    };

    while !window.should_close() {
        chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3)
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
