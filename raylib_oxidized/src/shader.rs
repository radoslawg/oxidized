use std::ffi::c_void;

use crate::vector::{Vector3, Vector4};

#[derive(Debug)]
pub enum ShaderUniformValue {
    Float(f32),
    Int(i32),
    Vec3(Vector3),
    Vec4(Vector4),
}

#[repr(i32)]
enum ShaderUnformDataType {
    Float = 0, // Shader uniform type: float
    // Vec2 = 1,       // Shader uniform type: vec2 (2 float)
    Vec3 = 2, // Shader uniform type: vec3 (3 float)
    Vec4 = 3, // Shader uniform type: vec4 (4 float)
    Int = 4,  // Shader uniform type: int
              // Ivec2 = 5,      // Shader uniform type: ivec2 (2 int)
              // Ivec3 = 6,      // Shader uniform type: ivec3 (3 int)
              // Ivec4 = 7,      // Shader uniform type: ivec4 (4 int)
              // Uint = 8,       // Shader uniform type: unsigned int
              // Uivec2 = 9,     // Shader uniform type: uivec2 (2 unsigned int)
              // Uivec3 = 10,    // Shader uniform type: uivec3 (3 unsigned int)
              // Uivec4 = 11,    // Shader uniform type: uivec4 (4 unsigned int)
              // Sampler2d = 12, // Shader uniform type: sampler2d
}

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

    pub fn set_value(&self, uniform_name: &str, value: ShaderUniformValue) {
        unsafe {
            let location =
                raylib_ffi::GetShaderLocation(self.shader, raylib_ffi::rl_str!(uniform_name));
            match value {
                ShaderUniformValue::Float(v) => raylib_ffi::SetShaderValue(
                    self.shader,
                    location,
                    &v as *const _ as *const c_void,
                    ShaderUnformDataType::Float as i32,
                ),
                ShaderUniformValue::Int(v) => raylib_ffi::SetShaderValue(
                    self.shader,
                    location,
                    &v as *const _ as *const c_void,
                    ShaderUnformDataType::Int as i32,
                ),
                ShaderUniformValue::Vec3(v) => raylib_ffi::SetShaderValue(
                    self.shader,
                    location,
                    &v as *const _ as *const c_void,
                    ShaderUnformDataType::Vec3 as i32,
                ),
                ShaderUniformValue::Vec4(v) => raylib_ffi::SetShaderValue(
                    self.shader,
                    location,
                    &v as *const _ as *const c_void,
                    ShaderUnformDataType::Vec4 as i32,
                ),
            }
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
