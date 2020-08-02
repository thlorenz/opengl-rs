mod chapter;
extern crate glfw;

use glfw::Context;

extern crate gl;

extern crate nalgebra_glm as glm;

use opengl::shader::Shader;
use std::ffi::CString;
use std::ptr;

pub fn create_rectangle_vao() -> u32 {
    #[rustfmt::skip]
        let vertices: [f32;20] = [
        //    positions       texture
         0.5,  0.5, 0.0,   1.0, 1.0,   // top right
         0.5, -0.5, 0.0,   1.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0,   // bottom left
        -0.5,  0.5, 0.0,   0.0, 1.0    // top left
    ];
    #[rustfmt::skip]
        let indices = [
        0, 1, 3,
        1, 2, 3
    ];
    chapter::create_indexed_texture_vertices_vao(&vertices, &indices)
}

pub fn main() {
    let (mut ctx, mut window, events) = chapter::init_window();

    let shader = Shader::new(
        "src/01_getting_started/05_1_transformations.vert",
        "src/01_getting_started/05_1_transformations.frag",
    )
    .expect("Failed to create shader");

    let vao = create_rectangle_vao();

    let container_texture = chapter::load_texture("resources/textures/container.jpg", false, false);
    let smiley_texture = chapter::load_texture("resources/textures/awesomeface.png", true, true);

    let transform_field = CString::new("transform").unwrap();

    let translation_1 = glm::translate(&glm::Mat4::identity(), &glm::vec3(-0.5, 0.5, 0.0));
    let translation_2 = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.5, -0.5, 0.0));

    unsafe {
        shader.use_program();

        shader.set_int(&CString::new("containerTexture").unwrap(), 0);
        shader.set_int(&CString::new("smileyTexture").unwrap(), 1);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, container_texture);
        gl::ActiveTexture(gl::TEXTURE0 + 1);
        gl::BindTexture(gl::TEXTURE_2D, smiley_texture);
    }
    window.set_focus_on_show(true);
    while !window.should_close() {
        chapter::process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);

            let time = ctx.get_time() as f32;
            let vec_scale = glm::sin(&glm::vec3(time, time, 0.0));
            let scale = glm::scale(&translation_1, &vec_scale);
            shader.set_mat4(&transform_field, &scale);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            let rotation = glm::rotate(
                &translation_2,
                (time * 10.0).to_radians(),
                &glm::vec3(0.0, 0.0, 1.0),
            );
            shader.set_mat4(&transform_field, &rotation);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.swap_buffers();
        ctx.poll_events();
    }
}
