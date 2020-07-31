// https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/3.2.shaders_interpolation/shaders_interpolation.cpp

mod chapter;
extern crate glfw;

use glfw::Context;

extern crate gl;

use opengl::shader::Shader;

fn create_colored_triangle_vao() -> u32 {
    #[rustfmt::skip]
    let vertices: [f32;18] = [
        // position        // color
        0.5, -0.5, 0.0,    1.0, 0.0, 0.0,  // bottom right
       -0.5, -0.5, 0.0,    0.0, 1.0, 0.0,  // bottom left
        0.0,  0.5, 0.0,    0.0, 0.0, 1.0,  // top center
    ];
    chapter::create_vertices_vao(&vertices, 6)
}

pub fn main() {
    let (mut ctx, mut window, events) = chapter::init_window();

    let shader = Shader::new(
        "src/01_getting_started/03_2_shaders_interpolation.vert",
        "src/01_getting_started/03_2_shaders_interpolation.frag",
    )
    .expect("Failed to create shader");

    let vao = create_colored_triangle_vao();
    unsafe {
        shader.use_program();
    }

    while !window.should_close() {
        chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
