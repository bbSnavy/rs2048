use raylib::prelude::*;

mod board;
mod vector;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .resizable()
        .title("rs2048")
        .build();

    rl.set_target_fps(120);

    let board = board::Board::new();

    while !rl.window_should_close() {
        match rl.get_key_pressed() {
            None => {}
            Some(key) => {
                board.handle(key)
            }
        }

        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(Color::BLACK);

        board.render(&mut draw);

        draw.draw_fps(8, 8);
    }
}
