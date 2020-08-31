use glfw::Context;
use nalgebra_glm as glm;
use opengl::c_str;
use opengl::{
    camera::Camera,
    ch04_advanced_opengl::{
        create_skybox_vao, create_textured_cube_vao, create_textured_plane_vao,
    },
    cubemap::load_cubemap,
    scene,
    shader::Shader,
    util::load_texture,
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
        skybox_shader,
        cube_vao,
        plane_vao,
        skybox_vao,
        cube_texture,
        floor_texture,
        cubemap_texture,
    ) = unsafe {
        let cube_vao = create_textured_cube_vao();
        let plane_vao = create_textured_plane_vao();
        let skybox_vao = create_skybox_vao();

        let cube_texture = load_texture("resources/textures/container.jpg", Default::default());
        let floor_texture = load_texture("resources/textures/metal.png", Default::default());
        let cubemap_texture = load_cubemap((
            "resources/textures/skybox/right.jpg",
            "resources/textures/skybox/left.jpg",
            "resources/textures/skybox/top.jpg",
            "resources/textures/skybox/bottom.jpg",
            "resources/textures/skybox/back.jpg",
            "resources/textures/skybox/front.jpg",
        ));

        let shader = Shader::new(
            "src/ch04_advanced_opengl/06_1_cubemaps_skybox/shader.vert",
            "src/ch04_advanced_opengl/06_1_cubemaps_skybox/shader.frag",
        )
        .expect("Failed to create shader");

        shader.use_program();
        shader.set_int(c_str!("texture1"), 0);

        let skybox_shader = Shader::new(
            "src/ch04_advanced_opengl/06_1_cubemaps_skybox/skybox_shader.vert",
            "src/ch04_advanced_opengl/06_1_cubemaps_skybox/skybox_shader.frag",
        )
        .expect("Failed to create screen shader");

        skybox_shader.use_program();
        skybox_shader.set_int(c_str!("skybox"), 0);

        (
            shader,
            skybox_shader,
            cube_vao,
            plane_vao,
            skybox_vao,
            cube_texture,
            floor_texture,
            cubemap_texture,
        )
    };

    while !scene.window.should_close() {
        scene.update_camera();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);

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
            // Cube
            //

            gl::BindVertexArray(cube_vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, cube_texture);

            let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
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
            // Skybox
            //

            skybox_shader.use_program();
            let view_without_translation = glm::mat3_to_mat4(&glm::mat4_to_mat3(&view));
            skybox_shader.set_mat4(c_str!("projection"), &projection);
            skybox_shader.set_mat4(c_str!("view"), &view_without_translation);

            gl::DepthFunc(gl::LEQUAL);
            gl::DepthMask(gl::FALSE);
            gl::BindVertexArray(skybox_vao);
            {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_CUBE_MAP, cubemap_texture);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            gl::BindVertexArray(0);
            gl::DepthMask(gl::TRUE);
            gl::DepthFunc(gl::LESS);
        };

        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
