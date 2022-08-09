use raylib::prelude::*;
use std::ops::Deref;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    UP = 0,
    LEFT = 1,
    DOWN = 2,
    RIGHT = 3,
}

pub enum Gamestate {
    Play,
    GameOver,
    NextLevel,
    MainMenu,
    OptionMenu,
}

pub struct ResourceManager {
    pub mouse: Texture2D,
    pub cheese: Texture2D,
    pub spider: Texture2D,
    pub exit: Texture2D,
    pub title: Texture2D,
    pub start: Texture2D,
    pub options: Texture2D,
    pub final_score: Texture2D,
    pub ceasy: Texture2D,
    pub retry: Texture2D,
    pub back: Texture2D,
    pub volume_text: Texture2D,
    pub theme : Sound,
    pub clicksound : Sound,
    pub pickupsound : Sound,
}

impl ResourceManager {
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mouse = Self::load_texture(handle, thread, "mouse");
        let cheese = Self::load_texture(handle, thread, "cheese");
        let spider = Self::load_texture(handle, thread, "spider");
        let exit = Self::load_texture(handle, thread, "exit");
        let title = Self::load_texture(handle, thread, "title");
        let start = Self::load_texture(handle, thread, "start");
        let options = Self::load_texture(handle, thread, "options");
        let final_score = Self::load_texture(handle, thread, "finalscore");
        let ceasy = Self::load_texture(handle, thread, "cheasy");
        let retry = Self::load_texture(handle, thread, "retry");
        let back = Self::load_texture(handle, thread, "back");
        let volume_text = Self::load_texture(handle, thread, "volume_text");
        let theme = Self::load_audio("theme");
        let clicksound = Self::load_audio("click");
        let pickupsound = Self::load_audio("pickup");
        
        Self {
            mouse,
            cheese,
            spider,
            exit,
            title,
            start,
            options,
            final_score,
            ceasy,
            retry,
            back,
            volume_text,
            theme,
            clicksound,
            pickupsound,
        }
    }

    fn load_texture(handle: &mut RaylibHandle, thread: &RaylibThread, name: &str) -> Texture2D {
        handle
            .load_texture(thread, &format!("textures/{}.png", name))
            .expect(&format!("missing {} sprite", name))
    }

    fn load_font(handle: &mut RaylibHandle, thread: &RaylibThread, name: &str) -> Font {
        handle
            .load_font(thread, &format!("fonts/{}.ttf", name))
            .expect(&format!("missing {} font", name))
    }

    fn load_audio(name: &str) -> Sound {
        Sound::load_sound(&format!("audio/{}.ogg", name)).expect(&format!("missing {} audio file", name))
    }
    fn load_music(thread: &RaylibThread, name: &str) -> Music {
        Music::load_music_stream(thread, &format!("audio/{}.ogg", name)).expect(&format!("missing {} audio file", name))
    }
}

pub struct VisibleNode {
    pub pos: Vector2,
    pub size: Vector2,
    pub available: bool,
}

impl VisibleNode {
    pub fn create_vn(x: f32, y: f32, w: f32, h: f32) -> Self {
        VisibleNode {
            pos: Vector2 { x, y },
            size: Vector2 { x: w, y: h },
            available: true,
        }
    }
    pub fn draw(
        &self,
        frame_x: f32,
        frame_y: f32,
        drawer: &mut RaylibTextureMode<RaylibDrawHandle>,
        texture: &Texture2D,
    ) {
        if (!self.available) {
            return;
        }
        drawer.draw_texture_pro(
            texture,
            Rectangle {
                x: frame_x.round(),
                y: frame_y.round(),
                width: self.size.x,
                height: self.size.y,
            },
            Rectangle {
                x: self.pos.x.round(),
                y: self.pos.y.round(),
                width: self.size.x,
                height: self.size.y,
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
    }
    pub fn collision(&self, el_x: f32, el_y: f32, el_w: f32, el_h: f32) -> bool {
        return VisibleNode::box_collisions(
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y,
            el_x,
            el_y,
            el_w,
            el_h,
        );
    }
    fn box_collisions(
        x_1: f32,
        y_1: f32,
        w_1: f32,
        h_1: f32,
        x_2: f32,
        y_2: f32,
        w_2: f32,
        h_2: f32,
    ) -> bool {
        (x_2 < x_1 + w_1) && (x_2 + w_2 > x_1) && (y_2 < y_1 + h_1) && (y_2 + h_2 > y_1)
    }
}

pub struct AnimatedNode {
    pub node: VisibleNode,
    pub animation_steps_count: u8,
    pub animation_step: u8,
    pub animation_speed: u32,
    pub last_time_point: Instant,
}

impl AnimatedNode {
    pub fn create_an(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        animation_steps_count: u8,
        animation_speed: u32,
    ) -> Self {
        let last_time_point = Instant::now();
        let animation_step = 0;
        Self {
            node: VisibleNode::create_vn(x, y, w, h),
            animation_steps_count,
            animation_step,
            animation_speed,
            last_time_point,
        }
    }
    pub fn cycle_animation(&mut self) {
        if (self.last_time_point.elapsed().as_millis() > self.animation_speed as u128) {
            self.animation_step = (self.animation_step + 1) % self.animation_steps_count;
            self.last_time_point = Instant::now();
        }
    }
}

impl Deref for AnimatedNode {
    type Target = VisibleNode;
    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

pub struct AnimatedButton {
    pos: Vector2,
    size : Vector2,
    animation_duration: u32,
    animation_speed: u32,
    animation_status : bool,
    start : Instant,
    delta : Instant,
    pressed : bool,
    ready: bool,
}

impl AnimatedButton {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            pos : Vector2 {x, y},
            size : Vector2{x:w, y:h},
            animation_duration: 1000,
            animation_speed: 100,
            animation_status : true,
            start : Instant::now(),
            delta : Instant::now(),
            pressed: false,
            ready: false,
        }
    }
    fn cycle_animation(&mut self){
        if(self.start.elapsed().as_millis() >= self.animation_duration as u128){
            self.ready = true;
            return;
        }
        if(self.delta.elapsed().as_millis() >=self.animation_speed as u128){
            self.animation_status = !self.animation_status;
            self.delta = Instant::now();
        }
    }
    pub fn draw(&mut self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>, texture : &Texture2D){
        if(self.pressed && !self.ready){
            self.cycle_animation();
        }
        drawer.draw_texture_pro(
            texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: self.size.x,
                height: -self.size.y,
            },
            Rectangle {
                x: self.pos.x.round(),
                y: self.pos.y.round(),
                width: self.size.x,
                height: self.size.y,
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            if(self.animation_status){Color{r:0,g:167,b:255,a:255}}else{Color{r:255,g:103,b:0,a:255}},
        );
    }
    pub fn click(&mut self){
        self.start = Instant::now();
        self.pressed = true;
    }
    pub fn reset(&mut self){
        self.animation_status = true;
        self.ready = false;
        self.pressed = false;
    }
    pub fn is_ready(&mut self) -> bool{
        if(self.ready){
            self.reset();
            return true;
        }else{
            return false;
        }
    }
}

pub struct NumberDisplay{
    pos : Vector2,
    pub value : u32,
    backup : u32,
    start : Instant,
}

impl NumberDisplay{
    pub fn new(x : f32, y : f32) -> Self{
        Self{pos:Vector2 { x, y}, value:0, backup:0, start: Instant::now()}
    }
    fn cycle_animation(&mut self){
        if(self.backup != self.value && self.start.elapsed().as_millis() > 50){
            self.value += 1;
            self.start = Instant::now();
        }
    }
    pub fn escalate(&mut self){
        self.backup = self.value;
        self.value = 0;
        self.start = Instant::now();
    }
    pub fn has_escalated(&self) -> bool{
        return self.value == self.backup;
    }
    pub fn draw(&mut self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>){
        let text = self.value.to_string();
        let mut truepos = self.pos;
        if(self.backup != 0){
            self.cycle_animation();
        }
        for digit in text.chars(){
            if(digit != '1' && digit != '4'){
                drawer.draw_rectangle(truepos.x as i32 +1, truepos.y as i32, 2, 1, Color{r:255,g:103,b:0,a:255});
            }
            if(digit != '5' && digit != '6'){
                drawer.draw_rectangle(truepos.x as i32 +3, truepos.y as i32 - 2, 1, 2, Color{r:255,g:103,b:0,a:255});
            }
            if(digit != '2'){
                drawer.draw_rectangle(truepos.x as i32 +3, truepos.y as i32 - 5, 1, 2, Color{r:255,g:103,b:0,a:255});
            }
            if(digit != '1' && digit != '4' && digit != '7'){
                drawer.draw_rectangle(truepos.x as i32 +1, truepos.y as i32 - 6, 2, 1, Color{r:255,g:103,b:0,a:255});
            }
            if(digit == '0' || digit == '2' || digit == '6' || digit == '8'){
                drawer.draw_rectangle(truepos.x as i32, truepos.y as i32 - 5, 1, 2, Color{r:255,g:103,b:0,a:255});
            }
            if(digit != '1' && digit != '2' && digit != '3' && digit != '7'){
                drawer.draw_rectangle(truepos.x as i32, truepos.y as i32 - 2, 1, 2, Color{r:255,g:103,b:0,a:255});
            }
            if(digit != '0' && digit != '1' && digit != '7'){
                drawer.draw_rectangle(truepos.x as i32 +1, truepos.y as i32 - 3, 2, 1, Color{r:255,g:103,b:0,a:255});
            }
            truepos.x += 5.0;
        }
    }
    pub fn translate(&mut self, x : f32, y : f32){
        self.pos.x = x;
        self.pos.y = y;
    }
    pub fn reset(&mut self){
        self.value = 0;
        self.backup = 0;
    }
    pub fn get_pos(&self) -> &Vector2{
        return &self.pos;
    }
}
