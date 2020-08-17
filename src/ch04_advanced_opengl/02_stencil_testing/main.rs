use glfw::Context;
use nalgebra_glm as glm;
use opengl::c_str;
use opengl::{
    camera::Camera,
    ch04_advanced_opengl::{create_textured_cube_vao, create_textured_plane_vao},
    scene,
    shader::Shader,
    util::load_texture,
};
use std::ffi::CStr;

fn main() {
    let mut scene = scene::Scene::default();
    scene.camera = Camera {
        position: glm::vec3(0.0, 0.0, 3.0),
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();
    scene.move_window_to_left_monitor();

    let (shader, shader_single_color, cube_vao, plane_vao, cube_texture, floor_texture) = unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Enable(gl::STENCIL_TEST);
        gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
        gl::StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);

        let shader = Shader::new(
            "src/ch04_advanced_opengl/02_stencil_testing/shader.vert",
            "src/ch04_advanced_opengl/02_stencil_testing/shader.frag",
        )
        .expect("Failed to create shader");
        let shader_single_color = Shader::new(
            "src/ch04_advanced_opengl/02_stencil_testing/shader.vert",
            "src/ch04_advanced_opengl/02_stencil_testing/shader_single_color.frag",
        )
        .expect("Failed to create shader");

        let cube_vao = create_textured_cube_vao();
        let plane_vao = create_textured_plane_vao();

        let cube_texture = load_texture("resources/textures/marble.jpg", Default::default());
        let floor_texture = load_texture("resources/textures/metal.png", Default::default());

        shader.use_program();
        shader.set_int(c_str!("texture1"), 0);

        (
            shader,
            shader_single_color,
            cube_vao,
            plane_vao,
            cube_texture,
            floor_texture,
        )
    };

    while !scene.window.should_close() {
        scene.update_camera();

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            let projection =
                glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 100.0);
            let view = scene.camera.get_view();

            shader.use_program();
            shader.set_mat4(c_str!("projection"), &projection);
            shader.set_mat4(c_str!("view"), &view);
            shader.set_bool(c_str!("visualizeDepth"), scene.show_depth);

            shader_single_color.use_program();
            shader_single_color.set_mat4(c_str!("projection"), &projection);
            shader_single_color.set_mat4(c_str!("view"), &view);

            shader.use_program();
            //
            // Floor not written to stencil buffer
            //
            gl::StencilMask(0x00);
            gl::BindVertexArray(plane_vao);
            gl::BindTexture(gl::TEXTURE_2D, floor_texture);
            let model = glm::Mat4::identity();
            shader.set_mat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            //
            // Cubes to Stencil Buffer at normal size
            //
            gl::StencilFunc(gl::ALWAYS, 1, 0xff);
            gl::StencilMask(0xff);

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
            // Cubes to Render Buffer at slightly scaled size.
            // Only drawing parts not `1` resulting in borders, i.e. the object size differences
            //
            gl::StencilFunc(gl::NOTEQUAL, 1, 0xff);
            gl::StencilMask(0x00);
            gl::Disable(gl::DEPTH_TEST);

            let scale = 1.1;

            shader_single_color.use_program();
            gl::BindVertexArray(cube_vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, cube_texture);

            let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
            let model = glm::scale(&model, &glm::vec3(scale, scale, scale));
            shader_single_color.set_mat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(2.0, 0.0, 0.0));
            let model = glm::scale(&model, &glm::vec3(scale, scale, scale));
            shader_single_color.set_mat4(c_str!("model"), &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            gl::BindVertexArray(0);
            gl::StencilMask(0xff);
            gl::StencilFunc(gl::ALWAYS, 0, 0xFF);
            gl::Enable(gl::DEPTH_TEST);
        }
        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
