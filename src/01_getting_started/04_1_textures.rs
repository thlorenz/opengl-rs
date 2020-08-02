mod chapter;
extern crate glfw;

use glfw::Context;

extern crate gl;

use opengl::shader::Shader;
use std::ptr;

pub fn create_rectangle_vao() -> u32 {
    #[rustfmt::skip]
    let vertices: [f32;32] = [
    //    positions           colors       texture
        0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0,   // top right
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0,   // bottom right
       -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0,   // bottom left
       -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0    // top left
    ];
    #[rustfmt::skip]
    let indices = [
        0, 1, 3,
        1, 2, 3
    ];
    chapter::create_indexed_vertices_vao(&vertices, &indices, 8)
}

pub fn main() {
    let (mut ctx, mut window, events) = chapter::init_window();

    let shader = Shader::new(
        "src/01_getting_started/04_1_textures.vert",
        "src/01_getting_started/04_1_textures.frag",
    )
    .expect("Failed to create shader");

    let vao = create_rectangle_vao();

    let texture = chapter::load_texture("resources/textures/container.jpg", false, false);

    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture);
        shader.use_program();
    }

    window.set_focus_on_show(true);
    while !window.should_close() {
        chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}