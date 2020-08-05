use nalgebra_glm as glm;

use glfw::Context;
use opengl::c_str;
use opengl::camera::Camera;
use opengl::ch02_lighting::create_cube_with_normals_and_lamp_vaos;
use opengl::scene;
use opengl::shader::Shader;
use std::ffi::CStr;

fn main() {
    let mut scene = scene::Scene::default();
    scene.camera = Camera {
        position: glm::vec3(-1.4, 1.0, -1.85),
        pitch: -10.4,
        yaw: -306.0,
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();
    scene.move_window_to_left_monitor();

    let cube_shader = Shader::new(
        "src/ch02_lighting/02_basic_ex1/cube.vert",
        "src/ch02_lighting/02_basic_ex1/cube.frag",
    )
    .expect("Failed to create lighting shader");
    let lamp_shader = Shader::new(
        "src/ch02_lighting/02_basic_ex1/lamp.vert",
        "src/ch02_lighting/02_basic_ex1/lamp.frag",
    )
    .expect("Failed to create light cube shader");

    let (cube_vao, lamp_vao) = create_cube_with_normals_and_lamp_vaos();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    while !scene.window.should_close() {
        scene.update_camera();

        let time = scene.ctx.get_time() as f32;
        let x: f32 = 1.0 + time.sin() * 2.0;
        let y: f32 = (time / 2.0).sin();
        let light_pos = glm::vec3(x, y, 2.0);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection =
                glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 100.0);
            let view = scene.camera.get_view();

            // Cube with lighting
            cube_shader.use_program();
            cube_shader.set_vec3(c_str!("objectColor"), &glm::vec3(1.0, 0.5, 0.31));
            cube_shader.set_vec3(c_str!("lightColor"), &glm::vec3(1.0, 1.0, 1.0));
            cube_shader.set_vec3(c_str!("lightPos"), &light_pos);

            cube_shader.set_mat4(c_str!("projection"), &projection);
            cube_shader.set_mat4(c_str!("view"), &view);
            cube_shader.set_mat4(c_str!("model"), &glm::Mat4::identity());
            cube_shader.set_vec3(c_str!("camera"), &scene.camera.position);

            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // Lamp
            lamp_shader.use_program();
            lamp_shader.set_mat4(c_str!("projection"), &projection);
            lamp_shader.set_mat4(c_str!("view"), &view);

            let model = glm::translate(&glm::Mat4::identity(), &light_pos);
            let model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));
            lamp_shader.set_mat4(c_str!("model"), &model);

            gl::BindVertexArray(lamp_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
