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

    let cube_positions = chapter::cube_positions();

    let projection = glm::perspective(
        chapter::SCREEN_WIDTH as f32 / chapter::SCREEN_HEIGHT as f32,
        45.0_f32.to_radians(),
        0.1,
        100.0,
    );

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
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = -3.0;
    while !window.should_close() {
        let (w, a, s, d, q, e) = chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            if a {
                x += 0.1
            }
            if d {
                x -= 0.1
            }
            if e {
                y += 0.1
            }
            if q {
                y -= 0.1
            }
            if w {
                z += 0.1
            }
            if s {
                z -= 0.1
            }

            let view = glm::translate(&glm::Mat4::identity(), &glm::vec3(x, y, z));
            shader.set_mat4(&CString::new("view").unwrap(), &view);

            gl::BindVertexArray(vao);
            let time = ctx.get_time() as f32;
            for pos in cube_positions.iter() {
                let translated = glm::translate(&glm::Mat4::identity(), &pos);
                let model = glm::rotate(
                    &translated,
                    (20.0 * time).to_radians(),
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
