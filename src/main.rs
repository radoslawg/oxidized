use raylib::{
    self,
    color::Color,
    consts::TextureFilter,
    math::{Rectangle, Vector2},
    misc::AsF32,
    prelude::{RaylibDraw, RaylibTextureModeExt},
    texture::RaylibTexture2D,
};
use simplelog::TermLogger;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;
const RENDER_WIDTH: u32 = 320;
const RENDER_HEIGHT: u32 = 200;
const WINDOW_TITLE: &str = "Oxidize";
const SPLASH_SCREEN_PATH: &str = "assets/oxidized_splashscreen.png";

fn main() {
    TermLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    log::info!("-*-*-*- Oxidized Started -*-*-*-");

    log::debug!("Initializing Raylib Context...");
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(WINDOW_TITLE)
        .resizable()
        .build();

    let splash = rl
        .load_texture(&thread, SPLASH_SCREEN_PATH)
        .expect("Splash Screen not found");

    let mut target_texture = rl
        .load_render_texture(&thread, RENDER_WIDTH, RENDER_HEIGHT)
        .expect("Cannot create render texture");
    target_texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);
    while !rl.window_should_close() {
        rl.draw_texture_mode(&thread, &mut target_texture, |mut d| {
            d.clear_background(Color::BLACK);
            d.draw_texture_pro(
                &splash,
                // Negative height to flip the texture vertically because of how OpenGL works
                Rectangle::new(0., 0., 1920., 1080.),
                Rectangle::new(0., 0., RENDER_WIDTH.as_f32(), RENDER_HEIGHT.as_f32()),
                Vector2::new(0., 0.),
                0.,
                Color::WHITE,
            );
            // d.draw_texture(&splash, 0, 0, Color::WHITE);
            d.draw_text(
                "Oxidized",
                (WINDOW_WIDTH / 2) - 200,
                10,
                100,
                Color::CHOCOLATE,
            );
        });

        rl.draw(&thread, |mut d| {
            // Scale the render texture to fit the window
            d.draw_texture_pro(
                &target_texture,
                // Negative height to flip the texture vertically because of how OpenGL works
                Rectangle::new(0., 0., RENDER_WIDTH.as_f32(), -RENDER_HEIGHT.as_f32()),
                Rectangle::new(
                    0.,
                    0.,
                    d.get_render_width().as_f32(),
                    d.get_render_height().as_f32(),
                ),
                Vector2::new(0., 0.),
                0.,
                Color::WHITE,
            );
            d.draw_fps(0, 0);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_window_config() {
        assert_eq!(WINDOW_WIDTH, 1280);
        assert_eq!(WINDOW_HEIGHT, 720);
        assert_eq!(WINDOW_TITLE, "Oxidize");
    }

    #[test]
    fn verify_render_config() {
        assert_eq!(RENDER_WIDTH, 320);
        assert_eq!(RENDER_HEIGHT, 200);
    }

    #[test]
    fn it_has_splashscreen_asset() {
        assert!(
            std::path::Path::new(SPLASH_SCREEN_PATH).exists(),
            "Splash screen asset missing"
        );
    }
}
