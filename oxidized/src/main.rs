use anyhow::{Context, Result};
use raylib_oxidized::{
    camera3d::Camera3D,
    colors::*,
    light::Light,
    material::MaterialMapIndex,
    model::Model,
    shader::{Shader, ShaderUniformValue},
    vector::Vector3,
    window::Window,
    *,
};

pub fn main() -> Result<()> {
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
    let lights = [Light::new(
        Vector3 {
            x: 0.,
            y: 0.,
            z: 1.,
        },
        10.0,
        RED,
    )];
    let mut wall = Model::load_model("assets/models/BasicWall.gltf");
    let floor = Model::load_model("assets/models/floor.glb");
    let character = Model::load_model("assets/models/block_man.gltf");
    let woman = Model::load_model("assets/models/Block_Woman.gltf");
    let car = Model::load_model("assets/models/car.glb");
    let texture = load_texture("assets/colors/apollo.png");
    let light_shader =
        Shader::load_shader("assets/shaders/light.vert", "assets/shaders/light.frag");

    wall.get_material(0)
        .context("Material not found")?
        .set_texture(MaterialMapIndex::Albedo, texture);

    floor.set_shader(&light_shader);
    character.set_shader(&light_shader);
    woman.set_shader(&light_shader);
    car.set_shader(&light_shader);
    wall.set_shader(&light_shader);

    set_shader_lights(&light_shader, &lights);

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
                        floor.draw_model(Vector3 {
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
                wall.draw_model(Vector3 {
                    x: 4.0,
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
    Ok(())
}

fn set_shader_lights(shader: &Shader, lights: &[Light]) {
    shader.set_value(
        "num_lights",
        shader::ShaderUniformValue::Int(lights.len() as i32),
    );
    for (index, light) in lights.iter().enumerate() {
        shader.set_value(
            &format!("lights[{}].enabled", index),
            ShaderUniformValue::Int(1),
        );
        shader.set_value(
            &format!("lights[{}].position", index),
            ShaderUniformValue::Vec3(light.position),
        );
    }
}
