use raylib::prelude::*;

mod board;
mod vector;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .resizable()
        .msaa_4x()
        .title("rs2048")
        .build();

    rl.set_target_fps(360);

    let mut rect_pos = vector::Vector2::new(0, 0);
    let mut rect_vel = vector::Vector2::new(1, 1);

    let texture = rl
        .load_texture(&thread, "texture.png")
        .unwrap();

    let texture_x = texture.width;
    let texture_y = texture.height;

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(Color::BLACK);

        rect_pos = rect_pos.add(rect_vel.copy());
        if rect_pos.x < 0 || rect_pos.x >= draw.get_screen_width() {
            rect_vel = rect_vel.mul(vector::Vector2::new(-1, 1))
        }

        if rect_pos.y < 0 || rect_pos.y >= draw.get_screen_height() {
            rect_vel = rect_vel.mul(vector::Vector2::new(1, -1))
        }

        draw.draw_texture(
            &texture,
            rect_pos.x - texture_x / 2,
            rect_pos.y - texture_y / 2,
            Color::WHITE,
        );

        draw.draw_fps(8, 8);
    }
}
