mod chapter;
extern crate glfw;

use glfw::Context;

extern crate gl;

extern crate nalgebra_glm as glm;

use opengl::c_str;
use opengl::shader::Shader;
use std::ffi::CStr;

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

        shader.set_int(c_str!("containerTexture"), 0);
        shader.set_int(c_str!("smileyTexture"), 1);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, container_texture);
        gl::ActiveTexture(gl::TEXTURE0 + 1);
        gl::BindTexture(gl::TEXTURE_2D, smiley_texture);

        shader.set_mat4(c_str!("projection"), &projection);

        gl::Enable(gl::DEPTH_TEST);
    }

    window.set_focus_on_show(true);

    let mut camera_pos = glm::vec3(0.0, 0.0, 3.0);
    let camera_front = glm::vec3(0.0, 0.0, -1.0);
    let camera_up = glm::vec3(0.0, 1.0, 0.0);

    let mut ts = ctx.get_time();
    while !window.should_close() {
        let (w, a, s, d, ..) = chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let time = ctx.get_time();
            let dt = (time - ts) as f32;
            ts = time;

            let camera_speed = dt * 7.5;
            if w {
                camera_pos += camera_speed * &camera_front
            }
            if s {
                camera_pos -= camera_speed * &camera_front
            }
            if a {
                camera_pos -= glm::cross(&camera_front, &camera_up).normalize() * camera_speed;
            }
            if d {
                camera_pos += glm::cross(&camera_front, &camera_up).normalize() * camera_speed;
            }

            let camera_target = &camera_pos + &camera_front;
            let view = glm::look_at(&camera_pos, &camera_target, &camera_up);
            shader.set_mat4(c_str!("view"), &view);

            gl::BindVertexArray(vao);
            for i in 0..cube_positions.len() {
                let pos = &cube_positions[i];
                let translated = glm::translate(&glm::Mat4::identity(), &pos);
                let model = glm::rotate(
                    &translated,
                    (20.0 * (i as f32 + 1.0) * time as f32).to_radians(),
                    &glm::vec3(1.0, 0.3, 0.5).normalize(),
                );
                shader.set_mat4(c_str!("model"), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
