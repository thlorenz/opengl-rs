mod chapter;
extern crate glfw;

use glfw::Context;

extern crate gl;

extern crate nalgebra_glm as glm;

use opengl::shader::Shader;
use std::ffi::CString;
use std::ptr;

pub fn main() {
    let (mut ctx, mut window, events) = chapter::init_window();

    let shader = Shader::new(
        "src/01_getting_started/06_coordinate_systems.vert",
        "src/01_getting_started/06_coordinate_systems.frag",
    )
    .expect("Failed to create shader");

    let vao = chapter::create_box_vao();

    let container_texture = chapter::load_texture("resources/textures/container.jpg", false, false);
    let smiley_texture = chapter::load_texture("resources/textures/awesomeface.png", true, true);

    let view = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 0.0, -3.0));
    let projection = glm::perspective(
        45.0_f32.to_radians(),
        (chapter::SCREEN_WIDTH / chapter::SCREEN_HEIGHT) as f32,
        0.1,
        100.0,
    );

    unsafe {
        shader.use_program();

        shader.set_int(&CString::new("containerTexture").unwrap(), 0);
        shader.set_int(&CString::new("smileyTexture").unwrap(), 1);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, container_texture);
        gl::ActiveTexture(gl::TEXTURE0 + 1);
        gl::BindTexture(gl::TEXTURE_2D, smiley_texture);

        shader.set_mat4(&CString::new("view").unwrap(), &view);
        shader.set_mat4(&CString::new("projection").unwrap(), &projection);
    }

    window.set_focus_on_show(true);
    while !window.should_close() {
        chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let time = ctx.get_time() as f32;
            let model = glm::rotate(
                &glm::Mat4::identity(),
                (time * 100.0).to_radians(),
                &glm::vec3(0.5, 1.0, 0.0),
            );
            shader.set_mat4(&CString::new("model").unwrap(), &model);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
