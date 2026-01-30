use raylib::{self, color::Color, prelude::RaylibDraw};
use simplelog::TermLogger;

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;
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
        .build();

    let splash = rl
        .load_texture(&thread, SPLASH_SCREEN_PATH)
        .expect("Splash Screen not found");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&splash, 0, 0, Color::WHITE);
        d.draw_text("Oxidized", (1920 / 2) - 200, 10, 100, Color::CHOCOLATE);
        d.draw_text("Loading...", (1920 / 2) - 100, 200, 50, Color::LIGHTGRAY);
        d.draw_fps(2, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_window_config() {
        assert_eq!(WINDOW_WIDTH, 1920);
        assert_eq!(WINDOW_HEIGHT, 1080);
        assert_eq!(WINDOW_TITLE, "Oxidize");
    }

    #[test]
    fn it_has_splashscreen_asset() {
        assert!(
            std::path::Path::new(SPLASH_SCREEN_PATH).exists(),
            "Splash screen asset missing"
        );
    }
}
