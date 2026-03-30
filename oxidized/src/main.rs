use std::ffi::c_void;

use raylib_ffi::*;

const FLAG_MSAA_4X_HINT: u32 = 32;

struct Window {}

impl Window {
    fn new(width: u32, height: u32, title: &str) -> Window {
        unsafe {
            SetConfigFlags(FLAG_MSAA_4X_HINT);
            raylib_ffi::InitWindow(width as i32, height as i32, raylib_ffi::rl_str!(title));
        }
        Window {}
    }
    fn should_close(&self) -> bool {
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

fn clear_background(color: raylib_ffi::Color) {
    unsafe {
        raylib_ffi::ClearBackground(color);
    }
}

fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: raylib_ffi::Color) {
    unsafe {
        raylib_ffi::DrawText(raylib_ffi::rl_str!(text), pos_x, pos_y, font_size, color);
    }
}

fn draw<F: FnOnce()>(f: F) {
    unsafe {
        raylib_ffi::BeginDrawing();
        f();
        raylib_ffi::EndDrawing();
    }
}

fn mode_3d<F: FnOnce()>(camera: &Camera3D, f: F) {
    unsafe {
        raylib_ffi::BeginMode3D(camera.camera);
        f();
        raylib_ffi::EndMode3D();
    }
}
struct Camera3D {
    pub camera: raylib_ffi::Camera3D,
}

impl Camera3D {
    pub fn new(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Camera3D {
        Camera3D {
            camera: raylib_ffi::Camera3D {
                position,
                target,
                up,
                fovy,
                projection: 0,
            },
        }
    }
    pub fn update(&mut self) {
        unsafe {
            raylib_ffi::UpdateCamera(&mut self.camera, 0);
        }
    }
}

struct Model {
    model: raylib_ffi::Model,
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

struct Shader {
    shader: raylib_ffi::Shader,
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

pub fn main() {
    let window = Window::new(1600, 900, "Oxidized");
    let mut camera = Camera3D::new(
        Vector3 {
            x: 15.0,
            y: 5.0,
            z: 15.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        45.0,
    );
    let mut model = Model::load_model("d:/model.gltf");
    let mut character = Model::load_model("d:/block_man.gltf");
    let mut woman = Model::load_model("d:/block_woman.gltf");
    let mut car = Model::load_model("d:/car.glb");
    let light_shader = Shader::load_shader("d:/light.vert", "d:/light.frag");
    // let light_pos_loc = light_shader.get_shader_location("pointLightPos");

    for material in model.materials_mut() {
        material.shader = light_shader.shader;
    }
    for material in character.materials_mut() {
        material.shader = light_shader.shader;
    }
    for material in woman.materials_mut() {
        material.shader = light_shader.shader;
    }
    for material in car.materials_mut() {
        material.shader = light_shader.shader;
    }
    set_target_fps(120);
    // Render the window
    while !(window.should_close()) {
        let time = get_time();
        let my_light_pos = Vector3 {
            x: time.sin() as f32 * 10.0, // Moving back and forth on X
            y: 5.0,                      // Floating 5 units up
            z: time.cos() as f32 * 10.0, // Moving back and forth on Z
        };

        //light_shader.set_shader_value(light_pos_loc, my_light_pos);
        camera.update();
        draw(|| {
            // Render text and a background
            clear_background(colors::SKYBLUE);
            draw_text(
                "Oxidized! Now in Rust's safe mode!",
                210,
                200,
                20,
                colors::BLACK,
            );
            mode_3d(&camera, || {
                for x in -4..=6 {
                    for z in -4..=6 {
                        model.draw_model(Vector3 {
                            x: -4.0 * x as f32,
                            y: 0.0,
                            z: -4.0 * z as f32,
                        });
                    }
                }
                car.draw_model(Vector3 {
                    x: 0.0,
                    y: 0.5,
                    z: 0.0,
                });
                character.draw_model(Vector3 {
                    x: 12.0,
                    y: 0.5,
                    z: 8.0,
                });
                woman.draw_model(Vector3 {
                    x: 8.0,
                    y: 0.5,
                    z: 8.0,
                });
                // DrawGrid(20, 1.0);
            });
            draw_fps(10, 10);
        });
    }
}
