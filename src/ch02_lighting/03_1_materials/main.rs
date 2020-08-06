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
        position: glm::vec3(-2.0, -1.3, 3.3),
        pitch: 27.0,
        yaw: -408.0,
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();
    scene.move_window_to_left_monitor();

    let cube_shader = Shader::new(
        "src/ch02_lighting/03_1_materials/cube.vert",
        "src/ch02_lighting/03_1_materials/cube.frag",
    )
    .expect("Failed to create lighting shader");
    let lamp_shader = Shader::new(
        "src/ch02_lighting/03_1_materials/lamp.vert",
        "src/ch02_lighting/03_1_materials/lamp.frag",
    )
    .expect("Failed to create light cube shader");

    let (cube_vao, lamp_vao) = create_cube_with_normals_and_lamp_vaos();

    let light_pos = glm::vec3(1.2, 1.0, 2.0);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    while !scene.window.should_close() {
        scene.update_camera();

        let time = scene.ctx.get_time() as f32;
        let time_sin = time.sin();

        let base_light = glm::vec3(time_sin * 2.0, time_sin * 0.7, time_sin * 1.3);
        let light_diffuse = base_light * 0.5;
        let light_ambient = light_diffuse * 0.2;

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection =
                glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 100.0);
            let view = scene.camera.get_view();

            // Cube with lighting
            cube_shader.use_program();
            cube_shader.set_vec3(c_str!("material.ambient"), &glm::vec3(1.0, 0.5, 0.31));
            cube_shader.set_vec3(c_str!("material.diffuse"), &glm::vec3(1.0, 0.5, 0.31));
            cube_shader.set_vec3(c_str!("material.specular"), &glm::vec3(0.5, 0.5, 0.5));
            cube_shader.set_float(c_str!("material.shininess"), 32.0);

            cube_shader.set_vec3(c_str!("light.position"), &light_pos);

            cube_shader.set_vec3(c_str!("light.ambient"), &light_ambient);
            cube_shader.set_vec3(c_str!("light.diffuse"), &light_diffuse);
            cube_shader.set_vec3(c_str!("light.specular"), &glm::vec3(1.0, 1.0, 1.0));

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
