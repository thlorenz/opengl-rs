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

unsafe fn draw_scene(
    shader: &Shader,
    cube_vao: u32,
    plane_vao: u32,
    cube1_texture: u32,
    cube2_texture: u32,
    floor_texture: u32,
) {
    gl::Enable(gl::DEPTH_TEST);

    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    //
    // Cubes
    //
    gl::BindVertexArray(cube_vao);
    gl::ActiveTexture(gl::TEXTURE0);

    // In Front
    gl::BindTexture(gl::TEXTURE_2D, cube1_texture);
    let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-3.0, 0.0, -3.0));
    shader.set_mat4(c_str!("model"), &model);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);

    // In Back
    gl::BindTexture(gl::TEXTURE_2D, cube2_texture);
    let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(3.0, 0.0, 3.0));
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

fn main() {
    let mut scene = scene::Scene::default();
    scene.camera = Camera {
        position: glm::vec3(0.5, 0.5, 0.5),
        pitch: -4.5,
        yaw: -136.0,
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();
    scene.move_window_to_left_monitor();

    let (
        shader,
        mirror_shader,
        cube_vao,
        plane_vao,
        quad_vao,
        cube1_texture,
        cube2_texture,
        floor_texture,
        framebuffer,
        texture_color_buffer,
    ) = unsafe {
        let cube_vao = create_textured_cube_vao();
        let plane_vao = create_textured_plane_vao();
        let quad_vao = create_textured_quad_vao();

        let cube1_texture = load_texture("resources/textures/container.jpg", Default::default());
        let cube2_texture = load_texture("resources/textures/container2.png", Default::default());
        let floor_texture = load_texture("resources/textures/metal.png", Default::default());

        let shader = Shader::new(
            "src/ch04_advanced_opengl/05_ex1_mirror/shader.vert",
            "src/ch04_advanced_opengl/05_ex1_mirror/shader.frag",
        )
        .expect("Failed to create shader");

        shader.use_program();
        shader.set_int(c_str!("texture1"), 0);

        let mirror_shader = Shader::new(
            "src/ch04_advanced_opengl/05_ex1_mirror/mirror_shader.vert",
            "src/ch04_advanced_opengl/05_ex1_mirror/mirror_shader.frag",
        )
        .expect("Failed to create screen shader");

        mirror_shader.use_program();
        mirror_shader.set_int(c_str!("screenTexture"), 0);

        let (framebuffer, texture_color_buffer) =
            setup_texture_framebuffer(SCREEN_WIDTH, SCREEN_HEIGHT);

        (
            shader,
            mirror_shader,
            cube_vao,
            plane_vao,
            quad_vao,
            cube1_texture,
            cube2_texture,
            floor_texture,
            framebuffer,
            texture_color_buffer,
        )
    };

    while !scene.window.should_close() {
        scene.update_camera();

        let projection =
            glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 100.0);
        let view = scene.camera.get_view();
        unsafe {
            shader.use_program();
            shader.set_mat4(c_str!("projection"), &projection);
            shader.set_mat4(c_str!("view"), &view);
            shader.set_bool(c_str!("visualizeDepth"), scene.show_depth);

            // Draw Scene to default Buffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            draw_scene(
                &shader,
                cube_vao,
                plane_vao,
                cube1_texture,
                cube2_texture,
                floor_texture,
            );

            // Draw Scene looking back to framebuffer
            let view = scene.camera.get_back_view();
            shader.set_mat4(c_str!("view"), &view);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
            {
                draw_scene(
                    &shader,
                    cube_vao,
                    plane_vao,
                    cube1_texture,
                    cube2_texture,
                    floor_texture,
                );
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            // Bind Default Framebuffer and draw color texture as mirror
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            {
                gl::Disable(gl::DEPTH_TEST);

                gl::BindVertexArray(quad_vao);
                gl::BindTexture(gl::TEXTURE_2D, texture_color_buffer);

                mirror_shader.use_program();

                // Mirror on the left
                mirror_shader.set_bool(c_str!("leftMirror"), true);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);

                // Mirror on the right
                mirror_shader.set_bool(c_str!("leftMirror"), false);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);

                gl::BindVertexArray(0);
            }
        }

        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
