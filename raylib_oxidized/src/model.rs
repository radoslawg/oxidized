use raylib_ffi::*;

pub struct Model {
    pub model: raylib_ffi::Model,
}

impl Model {
    pub fn load_model(path: &str) -> Model {
        unsafe {
            let model: raylib_ffi::Model = raylib_ffi::LoadModel(raylib_ffi::rl_str!(path));
            Model { model }
        }
    }
    pub fn materials(&self) -> &[Material] {
        unsafe {
            std::slice::from_raw_parts(self.model.materials, self.model.materialCount as usize)
        }
    }
    pub fn materials_mut(&mut self) -> &mut [Material] {
        unsafe {
            std::slice::from_raw_parts_mut(self.model.materials, self.model.materialCount as usize)
        }
    }
    pub fn draw_model(&self, position: Vector3) {
        unsafe { DrawModel(self.model, position, 1.0, colors::WHITE) }
    }
}
