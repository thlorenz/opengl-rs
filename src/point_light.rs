use crate::shader::Shader;
use nalgebra_glm as glm;
use std::ffi::{CString, NulError};

pub struct PointLight {
    position: glm::Vec3,

    constant: f32,
    linear: f32,
    quadratic: f32,

    ambient: glm::Vec3,
    pub diffuse: glm::Vec3,
    pub specular: glm::Vec3,
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

    pub fn add_to_shader(&self, shader: &Shader, name: &str) -> Result<(), NulError> {
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

