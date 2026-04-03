use crate::{Vector3, colors, material::Material, shader::Shader};

pub struct Model {
    model: raylib_ffi::Model,
}

impl Model {
    pub fn load_model(path: &str) -> Model {
        unsafe {
            let model: raylib_ffi::Model = raylib_ffi::LoadModel(raylib_ffi::rl_str!(path));
            Model { model }
        }
    }
    pub fn get_material(&mut self, index: usize) -> Option<Material<'_>> {
        if index >= self.model.materialCount as usize {
            return None;
        }
        unsafe {
            let mat_ptr = self.model.materials.add(index);
            Some(Material::from_raw_mut(mat_ptr))
        }
    }
    pub fn draw_model(&self, position: Vector3) {
        unsafe { raylib_ffi::DrawModel(self.model, position, 1.0, colors::WHITE) }
    }

    pub fn set_shader(&self, shader: &Shader) {
        unsafe {
            let materials = std::slice::from_raw_parts_mut(
                self.model.materials,
                self.model.materialCount as usize,
            );
            for material in materials {
                material.shader = shader.shader;
            }
        }
    }
}
