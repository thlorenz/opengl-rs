mod chapter;
extern crate glfw;

use glfw::Context;

extern crate gl;

use opengl::shader::Shader;

pub fn main() {
    let (mut ctx, mut window, events) = chapter::init_window();

    let shader = Shader::new(
        "src/01_getting_started/03_2_shaders_interpolation_ex3.vert",
        "src/01_getting_started/03_2_shaders_interpolation_ex3.frag",
    )
    .expect("Failed to create shader");

    let vao = chapter::create_triangle_vao();

    window.set_focus_on_show(true);
    while !window.should_close() {
        chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader.use_program();

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
