mod drawable;
mod node;
use raylib::prelude::*;
use drawable::{Game, SCALE};



fn main() {
    let (mut r_handle, r_thread) = raylib::init()
        .size(64 * SCALE, 64 * SCALE)
        .title("Cheese Adventure")
        .build();
    let mut audio_device = RaylibAudio::init_audio_device();
    audio_device.set_master_volume(1.0);
    let mut game = Game::new(&mut r_handle, &r_thread);

    r_handle.set_target_fps(60);

    //main app loop
    while !r_handle.window_should_close() {
        game.update(&mut r_handle, &mut audio_device);
        //draw game
        let mut drawer = r_handle.begin_drawing(&r_thread);
        game.show(&mut drawer, &r_thread);
       
        //end draw game
    }
}
