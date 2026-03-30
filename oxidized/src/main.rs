use raylib_oxidized::{
    camera3d::{Camera3D, Vector3},
    model::Model,
    shader::Shader,
    window::Window,
    *,
};

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
