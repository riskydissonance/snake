use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 450).title("Snake").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGREEN);
        d.draw_text(
            "This is snake, but it's not done yet.",
            190,
            200,
            20,
            Color::LIGHTGRAY,
        );
    }
}
