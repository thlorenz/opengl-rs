mod chapter;
extern crate glfw;

use glfw::Context;

extern crate gl;

extern crate nalgebra_glm as glm;

use opengl::shader::Shader;
use std::ffi::CString;

pub fn main() {
    let (mut ctx, mut window, events) = chapter::init_window();

    let shader = Shader::new(
        "src/01_getting_started/06_coordinate_systems.vert",
        "src/01_getting_started/06_coordinate_systems.frag",
    )
    .expect("Failed to create shader");

    let vao = chapter::create_box_vao();

    let container_texture = chapter::load_texture("resources/textures/container.jpg", false, false);
    let smiley_texture = chapter::load_texture("resources/textures/awesomeface.png", false, true);

    let projection = glm::perspective(
        chapter::SCREEN_WIDTH as f32 / chapter::SCREEN_HEIGHT as f32,
        45.0_f32.to_radians(),
        0.1,
        100.0,
    );

    let cube_positions = chapter::cube_positions();

    unsafe {
        shader.use_program();

        shader.set_int(&CString::new("containerTexture").unwrap(), 0);
        shader.set_int(&CString::new("smileyTexture").unwrap(), 1);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, container_texture);
        gl::ActiveTexture(gl::TEXTURE0 + 1);
        gl::BindTexture(gl::TEXTURE_2D, smiley_texture);

        shader.set_mat4(&CString::new("projection").unwrap(), &projection);

        gl::Enable(gl::DEPTH_TEST);
    }

    window.set_focus_on_show(true);
    let radius = 10.0;
    let camera_target = glm::vec3(0.0, 0.0, 0.0);
    let camera_up = glm::vec3(0.0, 1.0, 0.0);

    while !window.should_close() {
        chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let time = ctx.get_time() as f32;
            let cam_x = time.sin() * radius;
            let cam_z = time.cos() * radius;
            let camera_pos = glm::vec3(cam_x, 0.0, cam_z);
            let view = glm::look_at(&camera_pos, &camera_target, &camera_up);
            shader.set_mat4(&CString::new("view").unwrap(), &view);

            gl::BindVertexArray(vao);
            for i in 0..cube_positions.len() {
                let pos = &cube_positions[i];
                let translated = glm::translate(&glm::Mat4::identity(), &pos);
                let model = glm::rotate(
                    &translated,
                    (20.0 * i as f32 * time as f32).to_radians(),
                    &glm::vec3(1.0, 0.3, 0.5).normalize(),
                );
                shader.set_mat4(&CString::new("model").unwrap(), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
