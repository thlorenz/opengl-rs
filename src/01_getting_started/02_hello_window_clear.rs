mod chapter;
extern crate glfw;

use glfw::Context;

extern crate gl;

pub fn main() {
    let (mut ctx, mut window, events) = chapter::init_window();

    while !window.should_close() {
        chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
