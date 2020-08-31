use glfw::Context;
use nalgebra_glm as glm;
use opengl::c_str;
use opengl::{
    camera::Camera,
    ch04_advanced_opengl::{
        create_normals_textured_cube_vao, create_skybox_vao, create_textured_plane_vao,
    },
    cubemap::load_cubemap,
    model::Model,
    scene,
    shader::Shader,
    util::load_texture,
};
use std::ffi::CStr;

fn main() {
    let mut scene = scene::Scene::default();
    scene.camera = Camera {
        position: glm::vec3(3.23, 16.93, 11.11),
        pitch: -31.69,
        yaw: -95.55,
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();
    scene.move_window_to_left_monitor();

    let (
        shader,
        skybox_shader,
        reflect_shader,
        refract_shader,
        cube_vao,
        plane_vao,
        skybox_vao,
        floor_texture,
        cubemap_texture,
        nano_model,
    ) = unsafe {
        let cube_vao = create_normals_textured_cube_vao();
        let plane_vao = create_textured_plane_vao();
        let skybox_vao = create_skybox_vao();
        let nano_model = Model::new("resources/objects/nanosuit/nanosuit.obj", true);

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
            "src/ch04_advanced_opengl/06_2_cubemaps_env_mapping/shader.vert",
            "src/ch04_advanced_opengl/06_2_cubemaps_env_mapping/shader.frag",
        )
        .expect("Failed to create shader");

        shader.use_program();
        shader.set_int(c_str!("texture1"), 0);

        let skybox_shader = Shader::new(
            "src/ch04_advanced_opengl/06_2_cubemaps_env_mapping/skybox_shader.vert",
            "src/ch04_advanced_opengl/06_2_cubemaps_env_mapping/skybox_shader.frag",
        )
        .expect("Failed to create screen shader");

        skybox_shader.use_program();
        skybox_shader.set_int(c_str!("skybox"), 0);

        let reflect_shader = Shader::new(
            "src/ch04_advanced_opengl/06_2_cubemaps_env_mapping/reflect_shader.vert",
            "src/ch04_advanced_opengl/06_2_cubemaps_env_mapping/reflect_shader.frag",
        )
        .expect("Failed to create reflect shader");

        reflect_shader.use_program();
        reflect_shader.set_int(c_str!("skybox"), 0);

        let refract_shader = Shader::new(
            "src/ch04_advanced_opengl/06_2_cubemaps_env_mapping/refract_shader.vert",
            "src/ch04_advanced_opengl/06_2_cubemaps_env_mapping/refract_shader.frag",
        )
        .expect("Failed to create refract shader");

        refract_shader.use_program();
        refract_shader.set_int(c_str!("skybox"), 0);

        (
            shader,
            skybox_shader,
            reflect_shader,
            refract_shader,
            cube_vao,
            plane_vao,
            skybox_vao,
            floor_texture,
            cubemap_texture,
            nano_model,
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

            //
            // Cube
            //
            refract_shader.use_program();
            refract_shader.set_mat4(c_str!("projection"), &projection);
            refract_shader.set_mat4(c_str!("view"), &view);
            refract_shader.set_vec3(c_str!("camera"), &scene.camera.position);

            gl::BindVertexArray(cube_vao);
            let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
            refract_shader.set_mat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            //
            // Model
            //
            reflect_shader.use_program();
            reflect_shader.set_mat4(c_str!("projection"), &projection);
            reflect_shader.set_mat4(c_str!("view"), &view);
            reflect_shader.set_vec3(c_str!("camera"), &scene.camera.position);

            let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(1.0, -0.5, -1.0));
            reflect_shader.set_mat4(c_str!("model"), &model);
            nano_model.draw(&reflect_shader);

            //
            // Floor
            //

            shader.use_program();
            shader.set_mat4(c_str!("projection"), &projection);
            shader.set_mat4(c_str!("view"), &view);
            shader.set_bool(c_str!("visualizeDepth"), scene.show_depth);

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
