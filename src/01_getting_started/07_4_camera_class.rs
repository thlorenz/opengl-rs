mod chapter;
extern crate glfw;

extern crate gl;

extern crate nalgebra_glm as glm;

use glfw::Context;
use opengl::shader::Shader;
use opengl::{scene, util};
use std::ffi::CString;

pub fn main() {
    let mut scene = scene::Scene::default();

    let shader = Shader::new(
        "src/01_getting_started/06_coordinate_systems.vert",
        "src/01_getting_started/06_coordinate_systems.frag",
    )
    .expect("Failed to create shader");

    let vao = chapter::create_box_vao();

    let container_texture = util::load_texture("resources/textures/container.jpg", false);
    let smiley_texture = util::load_texture("resources/textures/awesomeface.png", true);

    let cube_positions = chapter::cube_positions();

    unsafe {
        shader.use_program();

        shader.set_int(&CString::new("containerTexture").unwrap(), 0);
        shader.set_int(&CString::new("smileyTexture").unwrap(), 1);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, container_texture);
        gl::ActiveTexture(gl::TEXTURE0 + 1);
        gl::BindTexture(gl::TEXTURE_2D, smiley_texture);

        gl::Enable(gl::DEPTH_TEST);
    }

    let mut ts = scene.ctx.get_time();
    while !scene.window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let time = scene.ctx.get_time();
            let dt = (time - ts) as f32;
            ts = time;

            scene.process_events();
            scene.process_input(dt);

            let view = scene.camera.get_view();
            shader.set_mat4(&CString::new("view").unwrap(), &view);

            let projection =
                glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 100.0);

            shader.set_mat4(&CString::new("projection").unwrap(), &projection);

            gl::BindVertexArray(vao);
            for i in 0..cube_positions.len() {
                let pos = &cube_positions[i];
                let translated = glm::translate(&glm::Mat4::identity(), &pos);
                let model = glm::rotate(
                    &translated,
                    (20.0 * (i as f32 + 1.0) * time as f32).to_radians(),
                    &glm::vec3(1.0, 0.3, 0.5).normalize(),
                );
                shader.set_mat4(&CString::new("model").unwrap(), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
