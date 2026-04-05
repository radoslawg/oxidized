use raylib_ffi::*;
use std::{
    ffi::{CString, c_void},
    ptr,
};

pub struct Shader {
    pub shader: raylib_ffi::Shader,
}

impl Shader {
    pub fn load_shader(vertex_shader: Option<&str>, fragment_shader: &str) -> Shader {
        // 1. Allocate the C-strings and bind them to variables.
        // By keeping them in these variables, we guarantee they live until the end of the function.
        let vs_cstring = vertex_shader
            .map(|s| CString::new(s).expect("Null byte found in vertex shader string"));
        let fs_cstring =
            CString::new(fragment_shader).expect("Null byte found in fragment shader string");

        // 2. Extract the raw pointers safely.
        // We match on a REFERENCE to vs_cstring so we don't accidentally move and destroy it.
        let vs_ptr = match &vs_cstring {
            Some(c_str) => c_str.as_ptr(),
            None => ptr::null(), // The safe, idiomatic way to pass NULL to C
        };

        let fs_ptr = fs_cstring.as_ptr();

        // 3. Execute the FFI call
        unsafe {
            let shader = raylib_ffi::LoadShader(vs_ptr, fs_ptr);
            log::debug!("Loaded shader VS: {:?}, {}", vertex_shader, fragment_shader);
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

    pub fn shader_mode<F: FnOnce()>(&self, f: F) {
        unsafe {
            //TODO: This should be done once not with every frame?
            let target = LoadRenderTexture(GetScreenWidth(), GetScreenHeight());
            BeginTextureMode(target);
            f();
            EndTextureMode();
            raylib_ffi::BeginShaderMode(self.shader);
            DrawTextureRec(
                target.texture,
                Rectangle {
                    x: 0.,
                    y: 0.,
                    width: target.texture.width as f32,
                    height: -target.texture.height as f32,
                },
                Vector2 { x: 0., y: 0. },
                colors::WHITE,
            );
            raylib_ffi::EndShaderMode();
            UnloadRenderTexture(target);
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
