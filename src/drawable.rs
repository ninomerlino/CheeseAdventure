use raylib::prelude::*;
use KeyboardKey::*;
use rand::prelude::*;
use std::{fs, ops::Deref, vec, thread::Thread};


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
    pub exit : Texture2D,
}

impl TextureManager {
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mouse = Self::load(handle, thread, "mouse");
        let cheese = Self::load(handle, thread, "cheese");
        let spider = Self::load(handle, thread, "spider");
        let exit = Self::load(handle, thread, "exit");

        Self { mouse, cheese, spider, exit }
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
            if(self.pos.x > 0.0){
                self.direction = Direction::LEFT;
                self.pos.x -= self.speed;
            }
        } else if handle.is_key_down(KEY_D) {
            if(self.pos.x < 55.0){
                self.direction = Direction::RIGHT;
                self.pos.x += self.speed;
            }
        } else if handle.is_key_down(KEY_W) {
            if(self.pos.y < 55.0){
                self.direction = Direction::UP;
                self.pos.y += self.speed;
            }
        } else if handle.is_key_down(KEY_S) {
            if(self.pos.y > 0.0){
                self.direction = Direction::DOWN;
                self.pos.y -= self.speed;
            }
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
        Self { pos: Vector2 { x: p_x, y: p_y }, available: true, animation : 0, animation_rate : 50}
    }
    pub fn draw(&mut self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>,texture_manager : &TextureManager){
        self.animation = (self.animation + 1) % self.animation_rate;
        if(self.available){
            drawer.draw_texture_pro(
                &texture_manager.cheese,
                Rectangle {
                    x: (self.animation/10 * 5) as f32,
                    y: 0.0,
                    width: 5.0,
                    height: 5.0,
                },
                Rectangle {
                    x: self.pos.x,
                    y: self.pos.y,
                    width: 5.0,
                    height: 5.0,
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
    available : bool,
}

impl Spider{
    pub fn new(p_x : f32, p_y : f32, direction : Direction) -> Self{
        Spider { pos: Vector2{ x:p_x, y:p_y}, direction: direction, speed : 0.0, available: true }
    }
    pub fn draw(&mut self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>,texture_manager : &TextureManager){
        if(self.available){
            drawer.draw_texture_pro(
                &texture_manager.spider,
                Rectangle {
                    x: (self.direction as i32 * 6) as f32,
                    y: 0.0,
                    width: 6.0,
                    height: 6.0,
                },
                Rectangle {
                    x: self.pos.x.round(),
                    y: self.pos.y.round(),
                    width: 6.0,
                    height: 6.0,
                },
                Vector2 { x: 0.0, y: 0.0 },
                0.0,
                Color::WHITE,
            );
        }
    }
    pub fn cycle(&mut self, target : &Mouse){
        let orientation = Vector2{x : (target.pos.x - self.pos.x).round(), y : (target.pos.y - self.pos.y).round()};
        if(orientation.x.abs() >= orientation.y.abs()){
            if (orientation.x > 0.0) {
                if(self.pos.x < 58.0){
                    self.direction = Direction::RIGHT;
                    self.pos.x += self.speed;
                }
            }else{
                if(self.pos.x > 0.0){
                    self.direction = Direction::LEFT;
                    self.pos.x -= self.speed;
                }
            }
        }else{
            if (orientation.y > 0.0) {
                if(self.pos.y < 58.0){
                    self.direction = Direction::UP;
                    self.pos.y += self.speed;
                }
            }else{
                if(self.pos.y > 0.0){
                self.direction = Direction::DOWN;
                self.pos.y -= self.speed;
                }
            }
        }
    }
}

pub struct Exit{
    pub pos : Vector2,
    pub available : bool,
    animation: i32,
	animation_rate : i32,
}

impl Exit{
    pub fn new(p_x : f32, p_y : f32) -> Self{
        Self { pos: Vector2 { x: p_x, y: p_y }, available: false, animation : 0, animation_rate : 90}
    }
    pub fn draw(&mut self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>,texture_manager : &TextureManager){
        self.animation = (self.animation + 1) % self.animation_rate;
        if(self.available){
            drawer.draw_texture_pro(
                &texture_manager.exit,
                Rectangle {
                    x: (self.animation/10 * 5) as f32,
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

pub struct Level{
    mouse : Mouse,
    cheeses : Vec<Cheese>,
    spiders : Vec<Spider>,
    exit : Exit,
    points : u8,
    max_points : u8,
}

impl Level{
    pub fn generate(old_mouse_x : f32, old_mouse_y : f32, old_mouse_direction : Direction, enemy_spaw_rate : f32, cheese_spawn_rate : f32, rand : &mut ThreadRng) -> Self {
        use Direction::{UP,DOWN,RIGHT,LEFT};
        let new_direction = match old_mouse_direction{
            UP => DOWN,
            DOWN => UP,
            RIGHT => LEFT,
            LEFT => RIGHT,
        };
        let mouse = Mouse::new(old_mouse_x, old_mouse_y,new_direction);
        let mut cheeses = Vec::<Cheese>::new();
        let mut spiders = Vec::<Spider>::new();
        let exit = Exit::new(0.0, 0.0);
        //need at least 1 cheese
        for y in 0..8 {
            for x in 0..8 {
                let (prob_c, prob_e) : (f32,f32) = rand.gen();
                if( prob_c <= cheese_spawn_rate){
                    cheeses.push(Cheese::new((x*8) as f32, (y*8) as f32));
                }else if(prob_e <= enemy_spaw_rate){
                    println!("new spider spawned");
                    spiders.push(Spider::new((x*8) as f32, (y*8) as f32, Direction::DOWN));
                }
            }
        }
        if(cheeses.len() == 0){
            let (x,y) : (f32,f32) = rand.gen();
            cheeses.push(Cheese::new(x*60.0, y*60.0 as f32));
        }
        let max_points = cheeses.len() as u8;
        Self{mouse,cheeses,spiders, exit, points:0, max_points}
    }
    pub fn draw(&mut self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>,texture_manager : &TextureManager){
        for cheese in &mut self.cheeses{
            cheese.draw(drawer, texture_manager);
        }
        self.mouse.draw(drawer, texture_manager);
        for spider in &mut self.spiders{
            spider.draw(drawer, texture_manager);
        }
    }
    pub fn cycle(&mut self, handle : &mut RaylibHandle){
        self.mouse.cycle(handle);
        {
            let spiders_positions : Vec<Vector2> = self.spiders.iter().map(|el| el.pos.clone()).collect();
            for spider in &mut self.spiders {
                spider.cycle(&self.mouse);
                for pos in &spiders_positions {
                    if(pos.x != spider.pos.x && pos.y != spider.pos.y && Level::box_collisions(spider.pos.x, spider.pos.y, 6.0, 6.0, pos.x, pos.y, 6.0, 6.0)){
                        if(spider.direction == Direction::UP){
                            spider.pos.y -= spider.speed;
                        }else if(spider.direction == Direction::DOWN){
                            spider.pos.y += spider.speed;
                        }else if(spider.direction == Direction::LEFT){
                            spider.pos.x += spider.speed;
                        }else{
                            spider.pos.x -= spider.speed;
                        }
                    }
                }
            }
            for pos in spiders_positions {
                if(Level::box_collisions(self.mouse.pos.x+3.0, self.mouse.pos.y+3.0, 3.0, 3.0, pos.x, pos.y, 6.0, 6.0)){
                    println!("game over")
                }
            }
        }
        for cheese in &mut self.cheeses {
            if(cheese.available && Level::box_collisions(self.mouse.pos.x+3.0, self.mouse.pos.y+3.0, 3.0, 3.0, cheese.pos.x, cheese.pos.y, 6.0, 6.0)){
                self.points += 1;
                cheese.available = false;
                println!("Points {0}/{1}",self.points,self.max_points)
            }
        }

    }
    fn box_collisions( x_1 : f32, y_1 : f32, w_1: f32, h_1 : f32, x_2 : f32, y_2 : f32, w_2: f32, h_2 : f32) -> bool{
        (x_2 < x_1 + w_1) && (x_2 + w_2 > x_1) && (y_2 < y_1 + h_1) && (y_2 + h_2 > y_1)
    }
}
