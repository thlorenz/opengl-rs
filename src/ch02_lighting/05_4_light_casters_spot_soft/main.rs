use nalgebra_glm as glm;

use glfw::Context;
use opengl::c_str;
use opengl::shader::Shader;
use opengl::{
    camera::Camera,
    ch02_lighting::{create_textured_cube_with_normals_vao, cube_positions},
    scene,
    util::load_texture,
};
use std::ffi::CStr;

fn main() {
    let mut scene = scene::Scene::default();
    scene.camera = Camera {
        position: glm::vec3(-1.10, -3.62, -0.80),
        pitch: 45.49,
        yaw: -445.31,
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();
    scene.move_window_to_left_monitor();

    let cube_shader = Shader::new(
        "src/ch02_lighting/05_4_light_casters_spot_soft/cube.vert",
        "src/ch02_lighting/05_4_light_casters_spot_soft/cube.frag",
    )
    .expect("Failed to create lighting shader");

    let cube_vao = create_textured_cube_with_normals_vao();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let diffuse_map = load_texture("resources/textures/container2.png", Default::default());
    let specular_map = load_texture(
        "resources/textures/container2_specular.png",
        Default::default(),
    );
    let diffuse_idx: u32 = 0;
    let specular_idx: u32 = 1;

    unsafe {
        cube_shader.use_program();
        cube_shader.set_int(c_str!("material.diffuse"), diffuse_idx as i32);
        cube_shader.set_int(c_str!("material.specular"), specular_idx as i32);
        cube_shader.set_float(c_str!("material.shininess"), 32.0);

        cube_shader.set_float(c_str!("light.cutOff"), 12.5_f32.to_radians().cos());
        cube_shader.set_float(c_str!("light.outerCutOff"), 17.5_f32.to_radians().cos());
        cube_shader.set_vec3(c_str!("light.ambient"), &glm::vec3(0.1, 0.1, 0.1));
        cube_shader.set_vec3(c_str!("light.diffuse"), &glm::vec3(0.8, 0.8, 0.8));
        cube_shader.set_vec3(c_str!("light.specular"), &glm::vec3(1.0, 1.0, 1.0));
        cube_shader.set_float(c_str!("light.constant"), 1.0);
        cube_shader.set_float(c_str!("light.linear"), 0.09);
        cube_shader.set_float(c_str!("light.quadratic"), 0.032);

        // Diffuse Map
        gl::ActiveTexture(gl::TEXTURE0 + diffuse_idx);
        gl::BindTexture(gl::TEXTURE_2D, diffuse_map);
        // Specular Map
        gl::ActiveTexture(gl::TEXTURE0 + specular_idx);
        gl::BindTexture(gl::TEXTURE_2D, specular_map);
    }

    while !scene.window.should_close() {
        scene.update_camera();

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection =
                glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 100.0);
            let view = scene.camera.get_view();

            cube_shader.use_program();

            cube_shader.set_vec3(c_str!("light.position"), &scene.camera.position);
            cube_shader.set_vec3(c_str!("light.direction"), &scene.camera.front);

            cube_shader.set_mat4(c_str!("projection"), &projection);
            cube_shader.set_mat4(c_str!("view"), &view);
            cube_shader.set_vec3(c_str!("camera"), &scene.camera.position);

            gl::BindVertexArray(cube_vao);
            for (i, position) in cube_positions().iter().enumerate() {
                let angle = 20.0 * i as f32;
                let translated = glm::translate(&glm::Mat4::identity(), &position);
                let model = glm::rotate(&translated, angle, &glm::vec3(1.0, 0.3, 0.5).normalize());
                cube_shader.set_mat4(c_str!("model"), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
