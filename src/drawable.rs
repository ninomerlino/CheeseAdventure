use raylib::prelude::*;
use KeyboardKey::*;
use std::fs;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    UP = 0,
    LEFT = 1,
    DOWN = 2,
    RIGHT = 3,
}

pub struct TextureManager {
    pub mouse: Texture2D,
    pub cheese: Texture2D,
    pub spider : Texture2D,
}

impl TextureManager {
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mouse = Self::load(handle, thread, "mouse");
        let cheese = Self::load(handle, thread, "cheese");
        let spider = Self::load(handle, thread, "spider");
        Self { mouse, cheese, spider }
    }

    fn load(handle: &mut RaylibHandle, thread: &RaylibThread, name: &str) -> Texture2D {
        handle
            .load_texture(thread, &format!("textures/{}.png", name))
            .expect(&format!("missing {} sprite", name))
    }
}

pub struct Mouse {
    pos: Vector2,
    direction: Direction,
    speed: f32,
    animation: i32,
	animation_rate : i32,
}

impl Mouse {
    pub fn new(p_x: f32, p_y: f32, direction: Direction) -> Self {
        return Self {
            pos: Vector2 { x: p_x, y: p_y },
            direction: direction,
            speed: 0.2,
            animation: 0,
			animation_rate: 50,
        };
    }
    pub fn cycle(&mut self, handle: &mut RaylibHandle) {
        if handle.is_key_down(KEY_A) {
            self.direction = Direction::LEFT;
            self.pos.x -= self.speed;
        } else if handle.is_key_down(KEY_D) {
            self.direction = Direction::RIGHT;
            self.pos.x += self.speed;
        } else if handle.is_key_down(KEY_W) {
            self.direction = Direction::UP;
            self.pos.y += self.speed;
        } else if handle.is_key_down(KEY_S) {
            self.direction = Direction::DOWN;
            self.pos.y -= self.speed;
        }
        self.animation = (self.animation + 1) % self.animation_rate;
    }
    pub fn draw(&self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>, texture_manager : &TextureManager) {
        drawer.draw_texture_pro(
            &texture_manager.mouse,
            Rectangle {
                x: (self.direction as i32 * 9) as f32,
                y: ({self.animation > self.animation_rate / 2} as i32 * 9) as f32,
                width: 9.0,
                height: 9.0,
            },
            Rectangle {
                x: self.pos.x,
                y: self.pos.y,
                width: 9.0,
                height: 9.0,
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
    }
}

pub struct Cheese{
    pos : Vector2,
    available : bool,
    animation: i32,
	animation_rate : i32,
}

impl Cheese{
    pub fn new(p_x : f32, p_y : f32) -> Self{
        Self { pos: Vector2 { x: p_x, y: p_y }, available: true, animation : 0, animation_rate : 100}
    }
    pub fn draw(&mut self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>,texture_manager : &TextureManager){
        self.animation = (self.animation + 1) % self.animation_rate;
        if(self.available){
            drawer.draw_texture_pro(
                &texture_manager.cheese,
                Rectangle {
                    x: (self.animation/20 * 4) as f32,
                    y: 0.0,
                    width: 4.0,
                    height: 4.0,
                },
                Rectangle {
                    x: self.pos.x,
                    y: self.pos.y,
                    width: 4.0,
                    height: 4.0,
                },
                Vector2 { x: 0.0, y: 0.0 },
                0.0,
                Color::WHITE,
            );
        }
    }
    pub fn is_available(&self) -> bool{
        return self.available;
    }
}

pub struct Spider{
    pos: Vector2,
    direction: Direction,
    speed: f32,
    animation: i32,
	animation_rate : i32,
    available : bool,
}
