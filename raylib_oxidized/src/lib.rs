use raylib_ffi::*;

use crate::camera3d::Camera3D;

pub mod camera3d;
pub mod colors;
pub mod consts;
pub mod material;
pub mod model;
pub mod shader;
pub mod texture2d;
pub mod vector;
pub mod window;

pub fn set_trace_log_level(level: consts::LogLevel) {
    unsafe {
        raylib_ffi::SetTraceLogLevel(level as i32);
    }
}
pub fn load_texture(path: &str) -> Texture2D {
    unsafe { raylib_ffi::LoadTexture(raylib_ffi::rl_str!(path)) }
}

pub fn clear_background(color: raylib_ffi::Color) {
    unsafe {
        raylib_ffi::ClearBackground(color);
    }
}

pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: raylib_ffi::Color) {
    unsafe {
        raylib_ffi::DrawText(raylib_ffi::rl_str!(text), pos_x, pos_y, font_size, color);
    }
}

pub fn draw<F: FnOnce()>(f: F) {
    unsafe {
        raylib_ffi::BeginDrawing();
        f();
        raylib_ffi::EndDrawing();
    }
}

pub fn mode_3d<F: FnOnce()>(camera: &Camera3D, f: F) {
    unsafe {
        raylib_ffi::BeginMode3D(camera.camera);
        f();
        raylib_ffi::EndMode3D();
    }
}

pub fn set_target_fps(fps: u32) {
    unsafe {
        SetTargetFPS(fps as i32);
    }
}

pub fn get_time() -> f64 {
    unsafe { raylib_ffi::GetTime() }
}

pub fn draw_fps(x: u32, y: u32) {
    unsafe {
        DrawFPS(x as i32, y as i32);
    }
}
