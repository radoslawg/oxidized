use raylib::{self, color::Color, prelude::RaylibDraw};

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Oxidize").build();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::SKYBLUE);
        d.draw_text("Hello from Raylib", 10, 200, 40, Color::BLACK);
        d.draw_fps(5, 5)
    }
}
