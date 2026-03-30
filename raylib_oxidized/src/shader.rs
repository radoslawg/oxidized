use raylib_ffi::*;
use std::ffi::c_void;

pub struct Shader {
    pub shader: raylib_ffi::Shader,
}

impl Shader {
    pub fn load_shader(vertex_shader: &str, fragment_shader: &str) -> Shader {
        unsafe {
            let shader = raylib_ffi::LoadShader(
                raylib_ffi::rl_str!(vertex_shader),
                raylib_ffi::rl_str!(fragment_shader),
            );
            Shader { shader }
        }
    }
    pub fn get_shader_location(&self, uniform_name: &str) -> i32 {
        unsafe { raylib_ffi::GetShaderLocation(self.shader, raylib_ffi::rl_str!(uniform_name)) }
    }

    pub fn set_shader_value(&self, uniform_location: i32, value: Vector3) {
        unsafe {
            raylib_ffi::SetShaderValue(
                self.shader,
                uniform_location,
                &value as *const _ as *const c_void,
                2,
            );
        }
    }

    pub fn set_shader_value_f32(&self, uniform_location: i32, value: f32) {
        unsafe {
            raylib_ffi::SetShaderValue(
                self.shader,
                uniform_location,
                &value as *const _ as *const c_void,
                0,
            );
        }
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            raylib_ffi::UnloadShader(self.shader);
        }
    }
}
