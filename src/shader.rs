use std::ffi::{CStr, CString};
use std::ptr;
use std::str;

extern crate gl;
use gl::types::*;
use std::error::Error;

extern crate nalgebra_glm as glm;

#[allow(dead_code)]
pub struct Shader {
    pub id: u32,
}

#[allow(dead_code)]
impl Shader {
    pub fn new(vertex_path: &str, frag_path: &str) -> Result<Shader, Box<dyn Error>> {
        let vert_buffer = std::fs::read(&vertex_path)?;
        let frag_buffer = std::fs::read(&frag_path)?;
        let shader_program = unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vert_buffer).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            check_for_errors(vertex_path, vertex_shader, gl::COMPILE_STATUS);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(frag_buffer).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            check_for_errors(frag_path, fragment_shader, gl::COMPILE_STATUS);

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            check_for_errors(
                &format!("[ {} + {} ]", vertex_path, frag_path),
                shader_program,
                gl::LINK_STATUS,
            );

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            shader_program
        };
        Ok(Shader { id: shader_program })
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }

    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_uint(&self, name: &CStr, value: u32) {
        gl::Uniform1ui(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_mat4(&self, name: &CStr, mat: &glm::Mat4) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }

    pub unsafe fn set_vec3(&self, name: &CStr, vec: &glm::Vec3) {
        gl::Uniform3fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            vec.as_ptr(),
        );
    }
}

fn check_for_errors(path: &str, item: u32, status_type: u32) {
    let mut success = gl::FALSE as GLint;
    let mut info_log: Vec<u8> = vec![0; 512];
    unsafe {
        info_log.set_len(512);
        if status_type == gl::COMPILE_STATUS {
            gl::GetShaderiv(item, status_type, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    item,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                eprintln!(
                    "Compilation failed of {}\n{}",
                    path,
                    str::from_utf8(&info_log).unwrap()
                );
            }
        } else if status_type == gl::LINK_STATUS {
            gl::GetProgramiv(item, status_type, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    item,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                eprintln!(
                    "Linking failed of {}\n{}",
                    path,
                    str::from_utf8(&info_log).unwrap()
                );
            }
        }
    }
}
