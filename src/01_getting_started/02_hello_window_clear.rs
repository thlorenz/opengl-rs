mod common;
extern crate glfw;

use glfw::Context;

extern crate gl;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub fn main() {
    let mut ctx = glfw::init(glfw::FAIL_ON_ERRORS.clone()).unwrap();
    ctx.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    ctx.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
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

    while !window.should_close() {
        common::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
