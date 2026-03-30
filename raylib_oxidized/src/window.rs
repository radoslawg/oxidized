use raylib_ffi::*;

pub struct Window {}

pub const FLAG_MSAA_4X_HINT: u32 = 32;

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        unsafe {
            SetConfigFlags(FLAG_MSAA_4X_HINT);
            raylib_ffi::InitWindow(width as i32, height as i32, raylib_ffi::rl_str!(title));
        }
        Window {}
    }
    pub fn should_close(&self) -> bool {
        unsafe { raylib_ffi::WindowShouldClose() }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            raylib_ffi::CloseWindow();
        }
    }
}
