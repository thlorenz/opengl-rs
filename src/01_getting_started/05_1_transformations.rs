mod chapter;
extern crate glfw;

use glfw::Context;

extern crate gl;

extern crate nalgebra_glm as glm;

use opengl::c_str;
use opengl::shader::Shader;
use std::ffi::CStr;
use std::ptr;

pub fn main() {
    let (mut ctx, mut window, events) = chapter::init_window();

    let shader = Shader::new(
        "src/01_getting_started/05_1_transformations.vert",
        "src/01_getting_started/05_1_transformations.frag",
    )
    .expect("Failed to create shader");

    let vao = chapter::create_rectangle_vao();

    let container_texture = chapter::load_texture("resources/textures/container.jpg", false, false);
    let smiley_texture = chapter::load_texture("resources/textures/awesomeface.png", true, true);

    let translation = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.5, -0.5, 0.0));

    unsafe {
        shader.use_program();

        shader.set_int(c_str!("containerTexture"), 0);
        shader.set_int(c_str!("smileyTexture"), 1);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, container_texture);
        gl::ActiveTexture(gl::TEXTURE0 + 1);
        gl::BindTexture(gl::TEXTURE_2D, smiley_texture);
    }
    window.set_focus_on_show(true);
    while !window.should_close() {
        chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let time = ctx.get_time() as f32;
            let rotation = glm::rotate(
                &translation,
                (time * 10.0).to_radians(),
                &glm::vec3(0.0, 0.0, 1.0),
            );
            shader.set_mat4(c_str!("transform"), &rotation);

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
