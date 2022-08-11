/*
Cheese Adventure a small and simple game
    Copyright (C) 2022  ninomerlino

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
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
