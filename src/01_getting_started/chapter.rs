extern crate glfw;

use glfw::{Action, Key};
use std::sync::mpsc::Receiver;

use glfw::{Context, Glfw, Window, WindowEvent};

extern crate gl;
use gl::types::*;

use std::str;
use std::{mem, ptr};

use image::GenericImageView;
use std::ffi::c_void;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

#[allow(dead_code)] // this is actually used in lots of places
pub fn init_window() -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
    let mut ctx = glfw::init(glfw::FAIL_ON_ERRORS.clone()).unwrap();
    ctx.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    ctx.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    ctx.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = ctx
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "Learn OpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Create Window");
    window.set_pos(-(SCREEN_WIDTH as i32), 0);
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    (ctx, window, events)
}

pub fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

#[allow(dead_code)] // this is actually used in lots of places
pub fn check_for_errors(item: u32, status_type: u32) {
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

#[allow(dead_code)]
pub fn create_vertices_vao(vertices: &[f32], el_size: i32) -> u32 {
    let has_color = el_size >= 6;
    let stride = el_size * mem::size_of::<GLfloat>() as GLsizei;
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

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
            if has_color {
                gl::VertexAttribPointer(
                    1,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    (3 * mem::size_of::<GLfloat>() as GLsizei) as *const c_void,
                );
                gl::EnableVertexAttribArray(1);
            }
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        vao
    }
}

#[allow(dead_code)]
pub fn create_indexed_vertices_vao(vertices: &[f32], indices: &[u32], el_size: i32) -> u32 {
    let has_color = el_size >= 6;
    let has_texture = el_size >= 8;
    let stride = el_size * mem::size_of::<GLfloat>() as GLsizei;
    unsafe {
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                &indices[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            if has_color {
                gl::VertexAttribPointer(
                    1,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    (3 * mem::size_of::<GLfloat>() as GLsizei) as *const c_void,
                );
                gl::EnableVertexAttribArray(1);
            }
            if has_texture {
                gl::VertexAttribPointer(
                    2,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    (6 * mem::size_of::<GLfloat>() as GLsizei) as *const c_void,
                );
                gl::EnableVertexAttribArray(2);
            }

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
        vao
    }
}

#[allow(dead_code)]
pub fn load_texture(path: &str) -> u32 {
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Wrapping
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        // Filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let img = image::open(path).expect("Failed to load texture image");
        let texture_data = img.to_bytes();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &texture_data[0] as *const u8 as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    texture
}

#[allow(dead_code)]
pub fn create_triangle_vao() -> u32 {
    #[rustfmt::skip]
    let vertices: [f32;9] = [
       -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0
    ];
    create_vertices_vao(&vertices, 3)
}
