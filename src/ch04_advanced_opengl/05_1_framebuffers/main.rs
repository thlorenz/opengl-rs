use glfw::Context;
use nalgebra_glm as glm;
use opengl::c_str;
use opengl::{
    camera::Camera,
    ch04_advanced_opengl::{
        create_textured_cube_vao, create_textured_plane_vao, create_textured_quad_vao,
        setup_texture_framebuffer,
    },
    scene,
    shader::Shader,
    util::load_texture,
};
use scene::{SCREEN_HEIGHT, SCREEN_WIDTH};
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
        screen_shader,
        cube_vao,
        plane_vao,
        quad_vao,
        cube_texture,
        floor_texture,
        framebuffer,
        texture_color_buffer,
    ) = unsafe {
        let cube_vao = create_textured_cube_vao();
        let plane_vao = create_textured_plane_vao();
        let quad_vao = create_textured_quad_vao();

        let cube_texture = load_texture("resources/textures/container.jpg", Default::default());
        let floor_texture = load_texture("resources/textures/metal.png", Default::default());

        let shader = Shader::new(
            "src/ch04_advanced_opengl/05_1_framebuffers/shader.vert",
            "src/ch04_advanced_opengl/05_1_framebuffers/shader.frag",
        )
        .expect("Failed to create shader");

        shader.use_program();
        shader.set_int(c_str!("texture1"), 0);

        let screen_shader = Shader::new(
            "src/ch04_advanced_opengl/05_1_framebuffers/screen_shader.vert",
            "src/ch04_advanced_opengl/05_1_framebuffers/screen_shader.frag",
        )
        .expect("Failed to create screen shader");

        screen_shader.use_program();
        screen_shader.set_int(c_str!("screenTexture"), 0);

        let (framebuffer, texture_color_buffer) =
            setup_texture_framebuffer(SCREEN_WIDTH, SCREEN_HEIGHT);

        (
            shader,
            screen_shader,
            cube_vao,
            plane_vao,
            quad_vao,
            cube_texture,
            floor_texture,
            framebuffer,
            texture_color_buffer,
        )
    };

    while !scene.window.should_close() {
        scene.update_camera();

        unsafe {
            // Bind to Framebuffer and draw scene to Color Texture
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
            {
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

                gl::BindVertexArray(0);
            }

            // Bind Default Framebuffer and draw color texture to quad plane
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            {
                gl::Disable(gl::DEPTH_TEST);
                gl::ClearColor(1.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                screen_shader.use_program();
                gl::BindVertexArray(quad_vao);
                gl::BindTexture(gl::TEXTURE_2D, texture_color_buffer);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);

                gl::BindVertexArray(0);
            }
        };

        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
