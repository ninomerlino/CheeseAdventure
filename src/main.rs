use raylib::prelude::*;

mod drawable;
use drawable::{Direction, Mouse, TextureManager, Cheese};

const SCALE: i32 = 15;

fn main() {
    let (mut r_handle, r_thread) = raylib::init()
        .size(64 * SCALE, 64 * SCALE)
        .title("Cheese Adventure")
        .build();
    let mut screen_texture = r_handle
        .load_render_texture(&r_thread, 64, 64)
        .expect("Cannot create main texture");

    let texture_manager = TextureManager::new(&mut r_handle, &r_thread);
    let mut mouse = Mouse::new(10.0, 10.0, Direction::DOWN);
    let mut cheese = Cheese::new(40.0,40.0);

    r_handle.set_target_fps(60);
    while !r_handle.window_should_close() {
        mouse.cycle(&mut r_handle);

        let mut drawer = r_handle.begin_drawing(&r_thread);
        {
            let mut texture_drawer = drawer.begin_texture_mode(&r_thread, &mut screen_texture);            
            texture_drawer.clear_background(Color::GRAY);
            cheese.draw(&mut texture_drawer, &texture_manager);
            mouse.draw(&mut texture_drawer, &texture_manager);
            
        }
        drawer.draw_texture_ex(
            &screen_texture,
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            SCALE as f32,
            Color::WHITE,
        )
    }
}
