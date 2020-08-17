use nalgebra_glm as glm;

use glfw::Context;
use opengl::c_str;
use opengl::shader::Shader;
use opengl::{
    camera::Camera,
    ch02_lighting::{create_textured_cube_with_normals_and_lamp_vaos, cube_positions},
    scene,
    util::load_texture,
};
use std::ffi::{CStr, CString, NulError};

struct PointLight {
    position: glm::Vec3,

    constant: f32,
    linear: f32,
    quadratic: f32,

    ambient: glm::Vec3,
    diffuse: glm::Vec3,
    specular: glm::Vec3,
}

impl Default for PointLight {
    fn default() -> Self {
        PointLight {
            position: glm::Vec3::identity(),

            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,

            ambient: glm::vec3(0.05, 0.05, 0.05),
            diffuse: glm::vec3(0.8, 0.8, 0.8),
            specular: glm::vec3(1.0, 1.0, 1.0),
        }
    }
}

impl PointLight {
    pub fn at(position: glm::Vec3) -> Self {
        let mut point_light = PointLight::default();
        point_light.position = position;
        point_light
    }

    pub fn add_to_shader(&self, shader: &Shader, idx: u32) -> Result<(), NulError> {
        let name = format!("pointLights[{}]", idx);
        let position = CString::new(format!("{}.position", name))?;

        let constant = CString::new(format!("{}.constant", name))?;
        let linear = CString::new(format!("{}.linear", name))?;
        let quadratic = CString::new(format!("{}.quadratic", name))?;

        let ambient = CString::new(format!("{}.ambient", name))?;
        let diffuse = CString::new(format!("{}.diffuse", name))?;
        let specular = CString::new(format!("{}.specular", name))?;

        unsafe {
            shader.set_vec3(&position, &self.position);
            shader.set_float(&constant, self.constant);
            shader.set_float(&linear, self.linear);
            shader.set_float(&quadratic, self.quadratic);
            shader.set_vec3(&ambient, &self.ambient);
            shader.set_vec3(&diffuse, &self.diffuse);
            shader.set_vec3(&specular, &self.specular);
        }

        Ok(())
    }
}

fn main() {
    let point_light_positions = [
        glm::vec3(0.7, 0.2, 2.0),
        glm::vec3(2.3, -3.3, -4.0),
        glm::vec3(-4.0, 2.0, -12.0),
        glm::vec3(0.0, 0.0, -3.0),
    ];
    let point_lights = point_light_positions.iter().map(|&pos| PointLight::at(pos));

    let mut scene = scene::Scene::default();
    scene.camera = Camera {
        position: glm::vec3(-1.10, -3.62, -0.80),
        pitch: 45.49,
        yaw: -445.31,
        ..Camera::default()
    };
    scene.camera.update_camera_vectors();
    scene.move_window_to_left_monitor();

    let cube_shader = Shader::new(
        "src/ch02_lighting/06_1_multiple_lights/cube.vert",
        "src/ch02_lighting/06_1_multiple_lights/cube.frag",
    )
    .expect("Failed to create lighting shader");
    let lamp_shader = Shader::new(
        "src/ch02_lighting/06_1_multiple_lights/lamp.vert",
        "src/ch02_lighting/06_1_multiple_lights/lamp.frag",
    )
    .expect("Failed to create lamp shader");

    let (cube_vao, lamp_vao) = create_textured_cube_with_normals_and_lamp_vaos();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let diffuse_map = load_texture("resources/textures/container2.png", Default::default());
    let specular_map = load_texture(
        "resources/textures/container2_specular.png",
        Default::default(),
    );
    let diffuse_idx: u32 = 0;
    let specular_idx: u32 = 1;

    unsafe {
        cube_shader.use_program();
        cube_shader.set_int(c_str!("material.diffuse"), diffuse_idx as i32);
        cube_shader.set_int(c_str!("material.specular"), specular_idx as i32);
        cube_shader.set_float(c_str!("material.shininess"), 32.0);

        // Directional Light
        cube_shader.set_vec3(c_str!("dirLight.direction"), &glm::vec3(-0.2, -1.0, -0.3));
        cube_shader.set_vec3(c_str!("dirLight.ambient"), &glm::vec3(0.05, 0.05, 0.05));
        cube_shader.set_vec3(c_str!("dirLight.diffuse"), &glm::vec3(0.4, 0.4, 0.4));
        cube_shader.set_vec3(c_str!("dirLight.specular"), &glm::vec3(0.5, 0.5, 0.5));

        // Point Lights
        for (i, point_light) in point_lights.enumerate() {
            point_light
                .add_to_shader(&cube_shader, i as u32)
                .expect("initializing  point light");
        }

        // Spotlight
        cube_shader.set_float(c_str!("spotLight.cutOff"), 12.5_f32.to_radians().cos());
        cube_shader.set_float(c_str!("spotLight.outerCutOff"), 17.5_f32.to_radians().cos());
        cube_shader.set_vec3(c_str!("spotLight.ambient"), &glm::vec3(0.1, 0.1, 0.1));
        cube_shader.set_vec3(c_str!("spotLight.diffuse"), &glm::vec3(0.8, 0.8, 0.8));
        cube_shader.set_vec3(c_str!("spotLight.specular"), &glm::vec3(1.0, 1.0, 1.0));
        cube_shader.set_float(c_str!("spotLight.constant"), 1.0);
        cube_shader.set_float(c_str!("spotLight.linear"), 0.09);
        cube_shader.set_float(c_str!("spotLight.quadratic"), 0.032);

        // Diffuse Map
        gl::ActiveTexture(gl::TEXTURE0 + diffuse_idx);
        gl::BindTexture(gl::TEXTURE_2D, diffuse_map);
        // Specular Map
        gl::ActiveTexture(gl::TEXTURE0 + specular_idx);
        gl::BindTexture(gl::TEXTURE_2D, specular_map);
    }

    while !scene.window.should_close() {
        scene.update_camera();

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection =
                glm::perspective(scene.ratio(), scene.camera.zoom.to_radians(), 0.1, 100.0);
            let view = scene.camera.get_view();

            //
            // Cubes
            //

            cube_shader.use_program();

            cube_shader.set_mat4(c_str!("projection"), &projection);
            cube_shader.set_mat4(c_str!("view"), &view);
            cube_shader.set_vec3(c_str!("viewPos"), &scene.camera.position);

            cube_shader.set_vec3(c_str!("spotLight.position"), &scene.camera.position);
            cube_shader.set_vec3(c_str!("spotLight.direction"), &scene.camera.front);

            gl::BindVertexArray(cube_vao);
            for (i, position) in cube_positions().iter().enumerate() {
                let angle = 20.0 * i as f32;
                let translated = glm::translate(&glm::Mat4::identity(), &position);
                let model = glm::rotate(&translated, angle, &glm::vec3(1.0, 0.3, 0.5).normalize());
                cube_shader.set_mat4(c_str!("model"), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            //
            // Lamps
            //

            lamp_shader.use_program();

            lamp_shader.set_mat4(c_str!("projection"), &projection);
            lamp_shader.set_mat4(c_str!("view"), &view);

            gl::BindVertexArray(lamp_vao);
            for position in &point_light_positions {
                let model = glm::translate(&glm::Mat4::identity(), &position);
                // Note that model.scale() does not work here
                let model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));
                lamp_shader.set_mat4(c_str!("model"), &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
        scene.window.swap_buffers();
        scene.ctx.poll_events();
    }
}
