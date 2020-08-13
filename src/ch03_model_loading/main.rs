use nalgebra_glm as glm;

use glfw::Context;
use opengl::c_str;
use opengl::shader::Shader;
use opengl::{camera::Camera, model::Model, scene};
use std::ffi::CStr;

fn main() {
    let mut scene = scene::Scene::default();
    scene.camera = Camera {
        position: glm::vec3(0.0, 0.0, 4.0),
        pitch: -3.58,
        yaw: -89.2,
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();
    scene.move_window_to_left_monitor();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let (shader, model) = {
        let shader = Shader::new(
            "src/ch03_model_loading/shader.vert",
            "src/ch03_model_loading/shader.frag",
        )
        .expect("Failed to create shader");

        // let model = Model::new("resources/objects/backpack/backpack.obj", false);
        // let model = Model::new("resources/objects/nanosuit/nanosuit.obj", true);
        // let model = Model::new("resources/objects/rock/rock.obj", false);
        let model = Model::new("resources/objects/planet/planet.obj", false);

        // unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) }
        (shader, model)
    };

    while !scene.window.should_close() {
        scene.update_camera();

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection =
                glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 100.0);
            let view = scene.camera.get_view();

            shader.use_program();
            shader.set_mat4(c_str!("projection"), &projection);
            shader.set_mat4(c_str!("view"), &view);

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
