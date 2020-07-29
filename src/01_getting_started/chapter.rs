extern crate glfw;

use glfw::{Action, Key};
use std::sync::mpsc::Receiver;

use glfw::{Context, Glfw, Window, WindowEvent};

use std::ptr;
use std::str;

extern crate gl;
use gl::types::*;

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
