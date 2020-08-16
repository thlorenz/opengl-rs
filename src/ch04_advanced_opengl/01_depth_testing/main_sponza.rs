use nalgebra_glm as glm;

use glfw::Context;
use opengl::c_str;
use opengl::shader::Shader;
use opengl::{camera::Camera, model::Model, scene};
use std::ffi::CStr;

fn main() {
    let mut scene = scene::Scene::new(2560, 1440);
    scene.camera = Camera {
        position: glm::vec3(162.45, 125.74, -12.38),
        pitch: -14.73,
        yaw: -180.38,
        mov_speed: 250.0,
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let (shader, model) = {
        let shader = Shader::new(
            "src/ch04_advanced_opengl/01_depth_testing/shader_sponza.vert",
            "src/ch04_advanced_opengl/01_depth_testing/shader_sponza.frag",
        )
        .expect("Failed to create shader");

        let model = Model::new("resources/objects/sponza/sponza.obj", true);
        (shader, model)
    };

    while !scene.window.should_close() {
        scene.update_camera();

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection =
                glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 10000.0);
            let view = scene.camera.get_view();

            shader.use_program();
            shader.set_mat4(c_str!("projection"), &projection);
            shader.set_mat4(c_str!("view"), &view);
            shader.set_bool(c_str!("visualizeDepth"), scene.show_depth);

            let model_position = glm::vec3(0.0, -1.75, 0.0);
            let translated = glm::translate(&glm::Mat4::identity(), &model_position);
            let scaled = glm::scale(&translated, &glm::vec3(0.2, 0.2, 0.2));
            shader.set_mat4(c_str!("model"), &scaled);
            model.draw(&shader);
        }
        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
