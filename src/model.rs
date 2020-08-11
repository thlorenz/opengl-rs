use crate::{
    mesh::{Mesh, Texture, TextureType, Vertex},
    util::load_texture,
};
use nalgebra_glm as glm;
use std::path::Path;
use tobj;

#[derive(Default)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub loaded_textures: Vec<Texture>,
    directory: String,
}

impl Model {
    pub fn new(file: &str) -> Self {
        let mut model = Model::default();
        model.load_model(file);
        model
    }

    fn load_model(&mut self, path: &str) {
        let path = Path::new(path);

        self.directory = path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_str()
            .unwrap()
            .into();

        let obj = tobj::load_obj(path, true);

        let (models, materials) = obj.unwrap();
        for model in models {
            let mesh = &model.mesh;
            let nvertices = mesh.positions.len() / 3;

            // Vertex Data
            let mut vertices: Vec<Vertex> = Vec::with_capacity(nvertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..nvertices {
                let pstart = i * 3;
                let nstart = i * 3;
                let tstart = i * 2;
                vertices.push(Vertex {
                    position: glm::vec3(p[pstart], p[pstart + 1], p[pstart + 2]),
                    normal: glm::vec3(n[nstart], n[nstart + 1], n[nstart + 2]),
                    tex_coords: glm::vec2(t[tstart], t[tstart + 1]),
                    ..Vertex::default()
                });
            }

            // Texture Material
            let mut textures: Vec<Texture> = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                // Diffuse Map
                if !material.diffuse_texture.is_empty() {
                    let texture =
                        self.load_material_texture(&material.diffuse_texture, TextureType::Diffuse);
                    textures.push(texture);
                }
                // Specular Map
                if !material.specular_texture.is_empty() {
                    let texture = self
                        .load_material_texture(&material.specular_texture, TextureType::Specular);
                    textures.push(texture);
                }
                // Normal Map
                if !material.normal_texture.is_empty() {
                    let texture =
                        self.load_material_texture(&material.normal_texture, TextureType::Normal);
                    textures.push(texture);
                }
            }
            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
    }

    fn load_material_texture(&mut self, file: &str, typ: TextureType) -> Texture {
        let full_path = format!("{}/{}", self.directory, file);
        let texture = self.loaded_textures.iter().find(|t| t.file == file);

        if let Some(texture) = texture {
            return texture.clone();
        };
        let id = load_texture(&full_path, true);
        let texture = Texture {
            id,
            typ,
            file: file.into(),
        };
        self.loaded_textures.push(texture.clone());
        texture
    }
}
