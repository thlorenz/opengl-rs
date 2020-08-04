extern crate nalgebra_glm as glm;

use glfw::Context;
use opengl::c_str;
use opengl::camera::Camera;
use opengl::ch02_lighting::create_box_vao;
use opengl::scene;
use opengl::shader::Shader;
use std::ffi::CStr;

fn main() {
    let mut scene = scene::Scene::default();
    scene.camera = Camera {
        position: glm::vec3(1.25, 1.2, 3.5),
        pitch: -15.0,
        yaw: -100.0,
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();
    scene.move_window_to_left_monitor();

    let lighting_shader = Shader::new(
        "src/ch02_lighting/01_1_colors/colors.vert",
        "src/ch02_lighting/01_1_colors/colors.frag",
    )
    .expect("Failed to create lighting shader");
    let lamp_shader = Shader::new(
        "src/ch02_lighting/01_1_colors/lamp.vert",
        "src/ch02_lighting/01_1_colors/lamp.frag",
    )
    .expect("Failed to create light cube shader");

    let cube_vao = create_box_vao();
    let light_cube_vao = create_box_vao();

    let light_pos = glm::vec3(1.2, 1.0, 2.0);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    while !scene.window.should_close() {
        scene.update_camera();

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection =
                glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 100.0);
            let view = scene.camera.get_view();

            // Cube with lighting
            lighting_shader.use_program();
            lighting_shader.set_vec3(c_str!("objectColor"), &glm::vec3(1.0, 0.5, 0.31));
            lighting_shader.set_vec3(c_str!("lightColor"), &glm::vec3(1.0, 1.0, 1.0));

            lighting_shader.set_mat4(c_str!("projection"), &projection);
            lighting_shader.set_mat4(c_str!("view"), &view);
            lighting_shader.set_mat4(c_str!("model"), &glm::Mat4::identity());

            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // Lamp
            lamp_shader.use_program();
            lamp_shader.set_mat4(c_str!("projection"), &projection);
            lamp_shader.set_mat4(c_str!("view"), &view);

            let model = glm::translate(&glm::Mat4::identity(), &light_pos);
            let model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));
            lamp_shader.set_mat4(c_str!("model"), &model);

            gl::BindVertexArray(light_cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
