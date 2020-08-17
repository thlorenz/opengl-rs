use glfw::Context;
use nalgebra_glm as glm;
use opengl::c_str;
use opengl::{
    camera::Camera,
    ch04_advanced_opengl::{
        create_textured_cube_vao, create_textured_plane_vao, create_textured_transparent_vao,
        vec3_transparent_pos,
    },
    scene,
    shader::Shader,
    util::{load_texture, LoadTextureOpts},
};
use std::ffi::CStr;

fn main() {
    let mut scene = scene::Scene::default();
    scene.camera = Camera {
        position: glm::vec3(1.31, 1.27, 3.07),
        pitch: -16.34,
        yaw: -105.45,
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();
    scene.move_window_to_left_monitor();

    let (
        shader,
        cube_vao,
        plane_vao,
        transparent_vao,
        cube_texture,
        floor_texture,
        transparent_texture,
        mut windows,
    ) = unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        let shader = Shader::new(
            "src/ch04_advanced_opengl/03_2_blending_sorted/shader.vert",
            "src/ch04_advanced_opengl/03_2_blending_sorted/shader.frag",
        )
        .expect("Failed to create shader");

        let cube_vao = create_textured_cube_vao();
        let plane_vao = create_textured_plane_vao();
        let transparent_vao = create_textured_transparent_vao();
        let windows = vec3_transparent_pos();

        let cube_texture = load_texture("resources/textures/marble.jpg", Default::default());
        let floor_texture = load_texture("resources/textures/metal.png", Default::default());
        let transparent_texture = load_texture(
            "resources/textures/window.png",
            LoadTextureOpts {
                vflip: true,
                clamp_alpha: true,
                ..Default::default()
            },
        );

        shader.use_program();
        shader.set_int(c_str!("texture1"), 0);

        (
            shader,
            cube_vao,
            plane_vao,
            transparent_vao,
            cube_texture,
            floor_texture,
            transparent_texture,
            windows,
        )
    };

    while !scene.window.should_close() {
        scene.update_camera();

        windows.sort_by(|a, b| {
            let distance_a = glm::length(&(scene.camera.position - a));
            let distance_b = glm::length(&(scene.camera.position - b));
            (&distance_b).partial_cmp(&distance_a).unwrap()
        });

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection =
                glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 100.0);
            let view = scene.camera.get_view();

            shader.use_program();
            shader.set_mat4(c_str!("projection"), &projection);
            shader.set_mat4(c_str!("view"), &view);
            shader.set_bool(c_str!("visualizeDepth"), scene.show_depth);

            //
            // Cubes
            //
            gl::BindVertexArray(cube_vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, cube_texture);

            let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
            shader.set_mat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(2.0, 0.0, 0.0));
            shader.set_mat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            //
            // Floor
            //
            gl::BindVertexArray(plane_vao);
            gl::BindTexture(gl::TEXTURE_2D, floor_texture);
            let model = glm::Mat4::identity();
            shader.set_mat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            //
            // Vegetation
            //
            gl::BindVertexArray(transparent_vao);
            gl::BindTexture(gl::TEXTURE_2D, transparent_texture);
            for v in &windows {
                let model = glm::translate(&glm::Mat4::identity(), v);
                shader.set_mat4(c_str!("model"), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }

            gl::BindVertexArray(0);
        }
        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
