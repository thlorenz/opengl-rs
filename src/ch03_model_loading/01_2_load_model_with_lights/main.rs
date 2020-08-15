use nalgebra_glm as glm;

use glfw::Context;
use opengl::c_str;
use opengl::shader::Shader;
use opengl::{
    camera::Camera, ch02_lighting::create_box_vao, model::Model, point_light::PointLight, scene,
};
use std::ffi::CStr;

fn main() {
    let mut scene = scene::Scene::default();
    scene.camera = Camera {
        position: glm::vec3(-10.18, 5.75, -7.20),
        pitch: 15.62,
        yaw: 32.98,
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();
    scene.move_window_to_left_monitor();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let (shader, suit_model, backpack_model, lamp_shader) = {
        let shader = Shader::new(
            "src/ch03_model_loading/01_2_load_model_with_lights/shader.vert",
            "src/ch03_model_loading/01_2_load_model_with_lights/shader.frag",
        )
        .expect("Failed to create shader");

        let lamp_shader = Shader::new(
            "src/ch03_model_loading/01_2_load_model_with_lights/lamp.vert",
            "src/ch03_model_loading/01_2_load_model_with_lights/lamp.frag",
        )
        .expect("Failed to create lamp shader");

        let suit_model = Model::new("resources/objects/nanosuit/nanosuit.obj", true);
        let backpack_model = Model::new("resources/objects/backpack/backpack.obj", false);
        // let model = Model::new("resources/objects/rock/rock.obj", false);
        // let model = Model::new("resources/objects/planet/planet.obj", false);
        (shader, suit_model, backpack_model, lamp_shader)
    };

    let lamp_vao = create_box_vao();
    let lamp_pos = glm::vec3(-3.0, 14.0, -2.0);
    let lamp_color = glm::vec3(0.1, 0.1, 8.0);
    let mut point_light = PointLight::at(lamp_pos);
    point_light.diffuse = lamp_color;
    point_light.specular = lamp_color * 0.6;

    unsafe {
        shader.use_program();

        // Directional Light
        shader.set_vec3(c_str!("dirLight.direction"), &glm::vec3(-0.2, -1.0, -0.3));
        shader.set_vec3(c_str!("dirLight.ambient"), &glm::vec3(0.05, 0.05, 0.05));
        shader.set_vec3(c_str!("dirLight.diffuse"), &glm::vec3(0.1, 0.1, 0.1));
        shader.set_vec3(c_str!("dirLight.specular"), &glm::vec3(0.2, 0.2, 0.2));

        // Point Light
        point_light
            .add_to_shader(&shader, "pointLight")
            .expect("initializing  point light");

        // Spotlight
        shader.set_float(c_str!("spotLight.cutOff"), 12.5_f32.to_radians().cos());
        shader.set_float(c_str!("spotLight.outerCutOff"), 17.5_f32.to_radians().cos());
        shader.set_vec3(c_str!("spotLight.ambient"), &glm::vec3(0.1, 0.1, 0.1));
        shader.set_vec3(c_str!("spotLight.diffuse"), &glm::vec3(3.8, 3.8, 2.8));
        shader.set_vec3(c_str!("spotLight.specular"), &glm::vec3(1.0, 1.0, 1.0));
        shader.set_float(c_str!("spotLight.constant"), 1.0);
        shader.set_float(c_str!("spotLight.linear"), 0.09);
        shader.set_float(c_str!("spotLight.quadratic"), 0.032);

        // Lamp Color
        lamp_shader.use_program();
        lamp_shader.set_vec3(c_str!("lampColor"), &lamp_color);
    }

    while !scene.window.should_close() {
        scene.update_camera();

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection =
                glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 100.0);
            let view = scene.camera.get_view();

            // Model
            shader.use_program();
            shader.set_mat4(c_str!("projection"), &projection);
            shader.set_mat4(c_str!("view"), &view);
            shader.set_vec3(c_str!("viewPos"), &scene.camera.position);
            shader.set_vec3(c_str!("spotLight.position"), &scene.camera.position);
            shader.set_vec3(c_str!("spotLight.direction"), &scene.camera.front);

            let model_position = glm::vec3(0.0, -1.75, 0.0);
            let translated = glm::translate(&glm::Mat4::identity(), &model_position);
            shader.set_mat4(c_str!("model"), &translated);
            suit_model.draw(&shader);

            let model_position = glm::vec3(0.0, 9.5, -2.8);
            let translated = glm::translate(&glm::Mat4::identity(), &model_position);
            let scaled = glm::scale(&translated, &glm::vec3(1.35, 1.35, 1.35));
            let rotated = glm::rotate(&scaled, 180_f32.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
            shader.set_mat4(c_str!("model"), &rotated);
            backpack_model.draw(&shader);

            // Lamp
            lamp_shader.use_program();
            lamp_shader.set_mat4(c_str!("projection"), &projection);
            lamp_shader.set_mat4(c_str!("view"), &view);
            let lamp = glm::translate(&glm::Mat4::identity(), &lamp_pos);
            lamp_shader.set_mat4(c_str!("model"), &lamp);

            gl::BindVertexArray(lamp_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
