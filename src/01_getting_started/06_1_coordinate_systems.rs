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
        "src/01_getting_started/06_coordinate_systems.vert",
        "src/01_getting_started/06_coordinate_systems.frag",
    )
    .expect("Failed to create shader");

    let vao = chapter::create_rectangle_vao();

    let container_texture = chapter::load_texture("resources/textures/container.jpg", false, false);
    let smiley_texture = chapter::load_texture("resources/textures/awesomeface.png", true, true);

    let model = glm::rotate(
        &glm::Mat4::identity(),
        (-55.0_f32).to_radians(),
        &glm::vec3(1.0, 0.0, 0.0),
    );
    let view = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 0.0, -3.0));
    // NOTE: arg order is different than the C++ counterpart: (aspect: N, fovy: N, near: N, far: N)
    let projection = glm::perspective(
        chapter::SCREEN_WIDTH as f32 / chapter::SCREEN_HEIGHT as f32,
        45.0_f32.to_radians(),
        0.1,
        100.0,
    );

    unsafe {
        shader.use_program();

        shader.set_int(c_str!("containerTexture"), 0);
        shader.set_int(c_str!("smileyTexture"), 1);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, container_texture);
        gl::ActiveTexture(gl::TEXTURE0 + 1);
        gl::BindTexture(gl::TEXTURE_2D, smiley_texture);

        shader.set_mat4(c_str!("model"), &model);
        shader.set_mat4(c_str!("view"), &view);
        shader.set_mat4(c_str!("projection"), &projection);
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
