use anyhow::{Context, Result};
use raylib_oxidized::{
    camera3d::*, colors::*, consts::*, light::*, material::*, model::*, shader::*, vector::*,
    window::*, *,
};
use simplelog::TermLogger;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const WINDOW_TITLE: &str = "Oxidize";

pub fn main() -> Result<()> {
    TermLogger::init(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .context("Cannot initilize Logger")?;
    set_trace_log_level(LogLevel::Warning);
    log::info!("-*-*-*- Oxidized Starting Up -*-*-*-");

    // Try to fix paths when running directly from the workspace root or the debug/release target directory
    if let Ok(exe_path) = std::env::current_exe()
        && let Some(exe_dir) = exe_path.parent()
        && exe_dir.join("resources").exists()
    {
        let _ = std::env::set_current_dir(exe_dir);
    }

    let window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE);
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
    let mut wall = Model::load_model("resources/models/BasicWall.gltf");
    let mut simple_wall = Model::load_model("resources/models/simple_wall.gltf");
    let mut floor = Model::load_model("resources/models/floor.glb");
    let mut character = Model::load_model("resources/models/block_man.gltf");
    let mut woman = Model::load_model("resources/models/Block_Woman.gltf");
    let mut car = Model::load_model("resources/models/car.glb");
    log::debug!("Models loaded");
    let texture = raylib_oxidized::load_texture("resources/colors/apollo.png");
    let light_shader = Shader::load_shader(
        "resources/shaders/light.vert",
        "resources/shaders/light.frag",
    );

    wall.get_material(0)
        .context("Material not found")?
        .set_texture(MaterialMapIndex::Albedo, texture);

    simple_wall
        .get_material(1)
        .context("Material not found")?
        .set_texture(MaterialMapIndex::Albedo, texture);
    simple_wall
        .get_material(2)
        .context("Material not found")?
        .set_texture(MaterialMapIndex::Emission, texture);

    floor.set_shader(&light_shader);
    character.set_shader(&light_shader);
    woman.set_shader(&light_shader);
    car.set_shader(&light_shader);
    wall.set_shader(&light_shader);
    simple_wall.set_shader(&light_shader);

    log::info!("Setup complete...");

    let character_pos = Vector3 {
        x: 12.0,
        y: 0.5,
        z: 8.0,
    };
    let woman_pos = Vector3 {
        x: 8.0,
        y: 0.5,
        z: 8.0,
    };
    let mut lights = vec![
        Light {
            position: character_pos,
            falloff: 5.0,
            color: YELLOW,
            rotation: None,
            direction: None,
        },
        Light {
            position: woman_pos,
            falloff: 10.0,
            color: SKYBLUE,
            rotation: None,
            direction: None,
        },
    ];
    simple_wall.with_position(Vector3 {
        x: 4.0,
        y: 0.5,
        z: 0.0,
    });
    lights.append(&mut simple_wall.get_lights());
    raylib_oxidized::set_target_fps(60);
    // Render the window
    while !(window.should_close()) {
        //light_shader.set_shader_value(light_pos_loc, my_light_pos);
        camera.update();
        draw(|| {
            // Render text and a background
            clear_background(SKYBLUE);
            draw_text("Oxidized! Now in Rust's safe mode!", 210, 200, 20, BLACK);
            mode_3d(&camera, || {
                for x in -4..=6 {
                    for z in -4..=6 {
                        floor
                            .with_position(Vector3 {
                                x: -4.0 * x as f32,
                                y: 0.0,
                                z: -4.0 * z as f32,
                            })
                            .draw_model();
                    }
                }
                car.with_position(Vector3 {
                    x: 0.0,
                    y: 0.5,
                    z: 0.0,
                })
                .draw_model();
                simple_wall.draw_model();
                character.with_position(character_pos).draw_model();
                woman.with_position(woman_pos).draw_model();
                set_shader_lights(&light_shader, &lights);
                // DrawGrid(20, 1.0);
            });
            draw_fps(10, 10);
        });
    }
    Ok(())
}

fn set_shader_lights(shader: &Shader, lights: &[Light]) {
    shader.set_value("numLights", ShaderUniformValue::Int(lights.len() as i32));
    for (index, light) in lights.iter().enumerate() {
        shader.set_value(
            &format!("lights[{}].enabled", index),
            ShaderUniformValue::Int(1),
        );
        shader.set_value(
            &format!("lights[{}].position", index),
            ShaderUniformValue::Vec3(light.position),
        );
        shader.set_value(
            &format!("lights[{}].falloff", index),
            ShaderUniformValue::Float(light.falloff),
        );
        shader.set_value(
            &format!("lights[{}].color", index),
            ShaderUniformValue::Vec4(color_to_vec4(light.color)),
        );
        shader.set_value(
            &format!("lights[{}].direction", index),
            ShaderUniformValue::Vec3(light.direction.unwrap_or(Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            })),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_correct_aspect_ration() {
        assert_eq!(WINDOW_WIDTH / WINDOW_HEIGHT, 16 / 9);
    }
}
