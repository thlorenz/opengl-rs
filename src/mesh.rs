use crate::shader::Shader;
use nalgebra_glm as glm;
use std::{
    ffi::{c_void, CString},
    fmt::Display,
    mem::size_of,
    ptr,
};

#[repr(C)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub tex_coords: glm::Vec2,
    pub tangent: glm::Vec3,
    pub bitangent: glm::Vec3,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: glm::vec3(0.0, 0.0, 0.0),
            normal: glm::vec3(0.0, 0.0, 0.0),
            tex_coords: glm::vec2(0.0, 0.0),
            tangent: glm::vec3(0.0, 0.0, 0.0),
            bitangent: glm::vec3(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Clone)]
pub enum TextureType {
    Diffuse,
    Specular,
    Normal,
    Height,
}

impl Display for TextureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureType::Diffuse => f.write_str("diff"),
            TextureType::Specular => f.write_str("spec"),
            TextureType::Normal => f.write_str("norm"),
            TextureType::Height => f.write_str("heig"),
        }?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Texture {
    pub id: u32,
    pub typ: TextureType,
    pub file: String,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub vao: u32,

    vbo: u32,
    ebo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let mut mesh = Self {
            vertices,
            indices,
            textures,
            vao: 0,
            vbo: 0,
            ebo: 0,
        };
        unsafe {
            mesh.setup_mesh();
        }
        mesh
    }

    pub unsafe fn draw(&self, shader: &Shader) {
        let mut diffuse = 0;
        let mut specular = 0;
        let mut normal = 0;
        let mut height = 0;

        for (i, texture) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);
            let (texture_idx, texture_name) = match texture.typ {
                TextureType::Diffuse => {
                    diffuse += 1;
                    (diffuse, "texture_diffuse")
                }
                TextureType::Specular => {
                    specular += 1;
                    (specular, "texture_specular")
                }
                TextureType::Normal => {
                    normal += 1;
                    (normal, "texture_normal")
                }
                TextureType::Height => {
                    height += 1;
                    (height, "texture_height")
                }
            };
            let sampler_2d_str = format!("{}{}", texture_name, texture_idx);
            let sampler_2d = CString::new(sampler_2d_str).unwrap();
            gl::Uniform1i(
                gl::GetUniformLocation(shader.id, sampler_2d.as_ptr()),
                i as i32,
            );
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
        }

        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as i32,
            gl::UNSIGNED_INT,
            ptr::null(),
        );

        gl::BindVertexArray(0);
        gl::ActiveTexture(gl::TEXTURE0);
    }

    unsafe fn setup_mesh(&mut self) {
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);
        gl::GenBuffers(1, &mut self.ebo);

        gl::BindVertexArray(self.vao);

        //
        // Vertex Buffer Object (vertices)
        //

        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        // Struct memory layout is sequential for all its items (repr(C)).
        // Thus we pass a pointer to the struct which translates to glm::vec3 or
        // glm::vec2 arrays which then translate to floats and then to a byte array.
        let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        //
        // Elements Buffer Object (indices)
        //

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        let size = (self.indices.len() * size_of::<u32>()) as isize;
        let data = &self.indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        //
        // Vertex Attributes
        //
        let size = size_of::<Vertex>() as i32;

        // position
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, position) as *const c_void,
        );

        // normal
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, normal) as *const c_void,
        );

        // texture coords
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, tex_coords) as *const c_void,
        );

        // tangent
        gl::EnableVertexAttribArray(3);
        gl::VertexAttribPointer(
            3,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, tangent) as *const c_void,
        );

        // bitangent
        gl::EnableVertexAttribArray(4);
        gl::VertexAttribPointer(
            4,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, bitangent) as *const c_void,
        );

        gl::BindVertexArray(0);
    }
}
