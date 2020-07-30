// https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/3.1.shaders_uniform/shaders_uniform.cpp

mod chapter;
extern crate glfw;

use glfw::Context;
use std::ffi::CString;

extern crate gl;

use opengl::shader::Shader;

pub fn main() {
    let (mut ctx, mut window, events) = chapter::init_window();

    let shader = Shader::new(
        "src/01_getting_started/04_0_shaders_uniform.vert",
        "src/01_getting_started/04_0_shaders_uniform.frag",
    )
    .expect("Failed to create shader");

    let vao = chapter::create_triangle_vao();

    while !window.should_close() {
        chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader.use_program();

            let time = ctx.get_time() as f32;
            let green = time.sin() / 2.0 + 0.5;
            let color = CString::new("ourColor").unwrap();
            let vertex_color_location = gl::GetUniformLocation(shader.id, color.as_ptr());
            gl::Uniform4f(vertex_color_location, 0.0, green, 0.0, 1.0);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
