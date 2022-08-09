use std::thread::Thread;

use crate::node::{
    AnimatedButton, AnimatedNode, Direction, Gamestate, NumberDisplay, ResourceManager, VisibleNode,
};
use rand::prelude::*;
use raylib::prelude::*;
use KeyboardKey::*;

pub const SCALE: i32 = 15;

pub struct Mouse {
    node: AnimatedNode,
    direction: Direction,
    speed: f32,
}

impl Mouse {
    pub fn new(x: f32, y: f32, direction: Direction) -> Self {
        Self {
            node: AnimatedNode::create_an(x, y, 9.0, 9.0, 8, 200),
            direction: direction,
            speed: 0.2,
        }
    }
    pub fn cycle(&mut self, handle: &mut RaylibHandle) {
        if handle.is_key_down(KEY_A) {
            if (self.node.pos.x > 0.0) {
                self.direction = Direction::LEFT;
                self.node.node.pos.x -= self.speed;
            }
        } else if handle.is_key_down(KEY_D) {
            if (self.node.pos.x < 55.0) {
                self.direction = Direction::RIGHT;
                self.node.node.pos.x += self.speed;
            }
        } else if handle.is_key_down(KEY_W) {
            if (self.node.pos.y < 55.0) {
                self.direction = Direction::UP;
                self.node.node.pos.y += self.speed;
            }
        } else if handle.is_key_down(KEY_S) {
            if (self.node.pos.y > 0.0) {
                self.direction = Direction::DOWN;
                self.node.node.pos.y -= self.speed;
            }
        }
    }
    pub fn draw(&mut self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>, texture: &Texture2D) {
        self.node.cycle_animation();
        self.node.draw(
            ((self.direction as i32) as f32) * self.node.size.x,
            (self.node.animation_step as f32) * self.node.size.x,
            drawer,
            texture,
        );
    }
    pub fn get_pos(&self) -> &Vector2 {
        &self.node.pos
    }
}

pub struct Cheese {
    node: AnimatedNode,
}

impl Cheese {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            node: AnimatedNode::create_an(x, y, 5.0, 5.0, 5, 200),
        }
    }
    pub fn draw(&mut self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>, texture: &Texture2D) {
        self.node.cycle_animation();
        self.node.draw(
            (self.node.animation_step as f32) * self.node.size.x,
            0.0,
            drawer,
            texture,
        )
    }
    pub fn is_available(&self) -> &bool {
        return &self.node.available;
    }
}

pub struct Spider {
    node: VisibleNode,
    direction: Direction,
    speed: f32,
}

impl Spider {
    pub fn new(x: f32, y: f32, direction: Direction) -> Self {
        Spider {
            node: VisibleNode::create_vn(x, y, 6.0, 6.0),
            direction: direction,
            speed: 0.08,
        }
    }
    pub fn draw(&mut self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>, texture: &Texture2D) {
        self.node.draw(
            (self.direction as i32 as f32) * self.node.size.x,
            0.0,
            drawer,
            texture,
        )
    }
    pub fn cycle(&mut self, target: &Vector2) {
        if (!self.node.available) {
            return;
        }
        let orientation = Vector2 {
            x: (target.x - self.node.pos.x).round(),
            y: (target.y - self.node.pos.y).round(),
        };
        if (orientation.x.abs() >= orientation.y.abs()) {
            if (orientation.x > 0.0) {
                if (self.node.pos.x < 58.0) {
                    self.direction = Direction::RIGHT;
                    self.node.pos.x += self.speed;
                }
            } else {
                if (self.node.pos.x > 0.0) {
                    self.direction = Direction::LEFT;
                    self.node.pos.x -= self.speed;
                }
            }
        } else {
            if (orientation.y > 0.0) {
                if (self.node.pos.y < 58.0) {
                    self.direction = Direction::UP;
                    self.node.pos.y += self.speed;
                }
            } else {
                if (self.node.pos.y > 0.0) {
                    self.direction = Direction::DOWN;
                    self.node.pos.y -= self.speed;
                }
            }
        }
    }
    pub fn get_pos(&self) -> &Vector2 {
        &self.node.pos
    }
    pub fn get_size(&self) -> &Vector2 {
        &self.node.size
    }
}

pub struct Exit {
    node: AnimatedNode,
    direction: Direction,
}

impl Exit {
    pub fn new(x: f32, y: f32) -> Self {
        let mut this = Self {
            node: AnimatedNode::create_an(x, y, 5.0, 7.0, 9, 50),
            direction: Direction::RIGHT,
        };
        this.node.node.available = false;
        return this;
    }
    pub fn draw(&mut self, drawer: &mut RaylibTextureMode<RaylibDrawHandle>, texture: &Texture2D) {
        self.node.cycle_animation();
        if (self.direction == Direction::RIGHT) {
            self.node.draw(
                (self.node.animation_step as f32) * self.node.size.x,
                0.0,
                drawer,
                texture,
            )
        } else {
            self.node.draw(
                (self.node.animation_step as f32) * self.node.size.x,
                self.node.size.y,
                drawer,
                texture,
            )
        }
    }
    pub fn is_available(&self) -> &bool {
        return &self.node.available;
    }
    pub fn activate(&mut self) {
        self.node.node.available = true;
    }
}

pub struct Level {
    cheeses: Vec<Cheese>,
    spiders: Vec<Spider>,
    exit: Exit,
    points: u8,
    max_points: u8,
}

impl Level {
    pub fn generate(
        forbidden_x: &f32,
        forbidden_y: &f32,
        enemy_spaw_rate: &f32,
        cheese_spawn_rate: &f32,
        rand: &mut ThreadRng,
    ) -> Self {
        let mut cheeses = Vec::<Cheese>::new();
        let mut spiders = Vec::<Spider>::new();
        let mut exit = Exit::new(10.0, 10.0);
        exit.node.node.pos.x = match rand.gen() {
            true => {
                exit.direction = Direction::LEFT;
                0.0
            }
            false => 59.0,
        };
        exit.node.node.pos.y = (rand.gen::<f32>() * 40.0) + 10.0;
        //need at least 1 cheese
        for y in 1..8 {
            for x in 1..8 {
                let (prob_c, prob_e): (f32, f32) = rand.gen();
                if (prob_c <= *cheese_spawn_rate) {
                    cheeses.push(Cheese::new((x * 8) as f32, (y * 8) as f32));
                } else if (prob_e <= *enemy_spaw_rate) {
                    let s_x = (x * 8) as f32;
                    let s_y = (y * 8) as f32;
                    if ((s_x - forbidden_x).powf(2.0) + (s_y - forbidden_y).powf(2.0)).sqrt() > 20.0
                    {
                        spiders.push(Spider::new(s_x, s_y, Direction::DOWN));
                    }
                }
            }
        }
        if (cheeses.len() == 0) {
            let (x, y): (f32, f32) = rand.gen();
            cheeses.push(Cheese::new(x * 60.0, y * 60.0 as f32));
        }
        let max_points = cheeses.len() as u8;
        Self {
            cheeses,
            spiders,
            exit,
            points: 0,
            max_points,
        }
    }
    pub fn cycle(&mut self, mouse: &Mouse, device : &mut RaylibAudio, pickup_sound : &Sound) -> Gamestate {
        {
            let spiders_positions: Vec<(Vector2, Vector2)> = self
                .spiders
                .iter()
                .map(|el| (el.get_pos().clone(), el.get_size().clone()))
                .collect();
            for spider in &mut self.spiders {
                spider.cycle(mouse.get_pos());
                for elm in &spiders_positions {
                    if (elm.0.x != spider.node.pos.x
                        && elm.0.y != spider.node.pos.y
                        && spider.node.collision(elm.0.x, elm.0.y, elm.1.x, elm.1.y))
                    {
                        if (spider.direction == Direction::UP) {
                            spider.node.pos.y -= spider.speed;
                        } else if (spider.direction == Direction::DOWN) {
                            spider.node.pos.y += spider.speed;
                        } else if (spider.direction == Direction::LEFT) {
                            spider.node.pos.x += spider.speed;
                        } else {
                            spider.node.pos.x -= spider.speed;
                        }
                    }
                }
                if (spider
                    .node
                    .collision(mouse.node.pos.x + 3.0, mouse.node.pos.y + 3.0, 3.0, 3.0))
                {
                    return Gamestate::GameOver;
                }
            }
        }
        if (!self.exit.is_available() && self.points == self.max_points) {
            self.exit.activate();
        } else if (*self.exit.is_available()
            && self
                .exit
                .node
                .collision(mouse.node.pos.x + 3.0, mouse.node.pos.y + 3.0, 3.0, 3.0))
        {
            return Gamestate::NextLevel;
        } else {
            for cheese in &mut self.cheeses {
                if (*cheese.is_available()
                    && cheese.node.collision(
                        mouse.node.pos.x + 3.0,
                        mouse.node.pos.y + 3.0,
                        mouse.node.size.x - 6.0,
                        mouse.node.size.y - 6.0,
                    ))
                {
                    device.play_sound(pickup_sound);
                    self.points += 1;
                    cheese.node.node.available = false;
                }
            }
        }
        return Gamestate::Play;
    }
}

pub struct Game {
    total_points: NumberDisplay,
    volume : NumberDisplay,
    enemy_spawn_rate: f32,
    cheese_spawn_rate: f32,
    game_state: Gamestate,
    level_count: u32,
    curr_level: Level,
    character: Mouse,
    rand_gen: ThreadRng,
    texture_manager: ResourceManager,
    screen_texture: RenderTexture2D,
    start_button: AnimatedButton,
    option_button: AnimatedButton,
    retry_button: AnimatedButton,
    extra_cheese: AnimatedButton,
    back: AnimatedButton,
}
impl Game {
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let screen_texture = handle
            .load_render_texture(&thread, 64, 64)
            .expect("Cannot create main texture");

        let mut texture_manager = ResourceManager::new(handle, thread);
        let mut rand_gen = thread_rng();
        let enemy_spawn_rate: f32 = 0.05;
        let cheese_spawn_rate: f32 = 0.02;
        let character = Mouse {
            node: AnimatedNode::create_an(10.0, 10.0, 9.0, 9.0, 8, 200),
            direction: Direction::UP,
            speed: 0.2,
        };
        let mut volume = NumberDisplay::new(40.0, 30.0);
        volume.value = 100;
        Self {
            total_points: NumberDisplay::new(1.0, 62.0),
            volume,
            enemy_spawn_rate,
            cheese_spawn_rate,
            game_state: Gamestate::MainMenu,
            level_count: 0,
            curr_level: Level::generate(
                &character.node.pos.x,
                &character.node.pos.y,
                &enemy_spawn_rate,
                &cheese_spawn_rate,
                &mut rand_gen,
            ),
            character,
            rand_gen,
            texture_manager,
            screen_texture,
            start_button: AnimatedButton::new(3.0, 20.0, 30.0, 7.0),
            option_button: AnimatedButton::new(3.0, 10.0, 36.0, 7.0),
            retry_button: AnimatedButton::new(5.0, 20.0, 30.0, 7.0),
            extra_cheese: AnimatedButton::new(0.0, 0.0, 64.0, 10.0),
            back: AnimatedButton::new(5.0, 5.0, 25.0, 7.0),
        }
    }
    pub fn update(&mut self, r_handle: &mut RaylibHandle, device : &mut RaylibAudio) {
        if(!device.is_sound_playing(&self.texture_manager.theme)){
            device.play_sound(&self.texture_manager.theme);
        }
        self.game_state = match self.game_state {
            Gamestate::Play => {
                self.character.cycle(r_handle);
                self.curr_level.cycle(&self.character, device, &self.texture_manager.pickupsound)
            }
            Gamestate::NextLevel => {
                self.level_count += 1;
                self.total_points.value += self.curr_level.max_points as u32;
                if (self.level_count % 8 == 0 && self.cheese_spawn_rate < 0.5) {
                    self.cheese_spawn_rate += 0.05;
                }
                if (self.level_count % 5 == 0 && self.enemy_spawn_rate < 0.5) {
                    self.enemy_spawn_rate += 0.05;
                }
                self.curr_level = Level::generate(
                    &self.character.node.pos.x,
                    &self.character.node.pos.y,
                    &self.enemy_spawn_rate,
                    &self.cheese_spawn_rate,
                    &mut self.rand_gen,
                );
                if (self.curr_level.spiders.len() == 0) {
                    self.extra_cheese.click();
                }
                Gamestate::Play
            }
            Gamestate::MainMenu => {
                if (self.start_button.is_ready()) {
                    Gamestate::Play
                } else if (self.option_button.is_ready()) {
                    Gamestate::OptionMenu
                } else if (r_handle.is_key_released(KEY_A)) {
                    device.play_sound(&self.texture_manager.clicksound);
                    self.start_button.click();
                    Gamestate::MainMenu
                } else if (r_handle.is_key_released(KEY_S)) {
                    device.play_sound(&self.texture_manager.clicksound);
                    self.option_button.click();
                    Gamestate::MainMenu
                } else {
                    Gamestate::MainMenu
                }
            }
            Gamestate::GameOver => {
                if (self.retry_button.is_ready()) {
                    self.reset_game();
                    Gamestate::Play
                } else {
                    if (self.total_points.get_pos().x == 1.0) {
                        self.total_points.translate(20.0, 45.0);
                        self.total_points.value *= self.level_count;
                        self.total_points.escalate();
                    }
                    if (r_handle.is_key_released(KEY_A) && self.total_points.has_escalated()) {
                        self.retry_button.click();
                        device.play_sound(&self.texture_manager.clicksound);
                    }
                    Gamestate::GameOver
                }
            }
            Gamestate::OptionMenu => if(self.back.is_ready()){
                Gamestate::MainMenu
            }else{
                if(r_handle.is_key_released(KEY_A)){
                    self.option_button.click();
                    self.back.click();
                }else if(r_handle.is_key_released(KEY_X)){
                    if(self.volume.value < 100){
                        self.volume.value += 10;
                        device.set_master_volume((self.volume.value as f32) / 100.0);
                    }
                }else if(r_handle.is_key_released(KEY_Z)){
                    if(self.volume.value > 0){
                        self.volume.value -= 10;
                        device.set_master_volume((self.volume.value as f32) / 100.0);
                    }
                }
                Gamestate::OptionMenu
            },
            _ => Gamestate::Play,
        };
    }
    pub fn show(&mut self, drawer: &mut RaylibDrawHandle, thread: &RaylibThread) {
        let mut texture_drawer = drawer.begin_texture_mode(thread, &mut self.screen_texture);
        texture_drawer.clear_background(Color {
            r: 70,
            g: 83,
            b: 98,
            a: 255,
        });
        match self.game_state {
            Gamestate::Play => {
                self.curr_level
                    .exit
                    .draw(&mut texture_drawer, &self.texture_manager.exit);
                for cheese in &mut self.curr_level.cheeses {
                    cheese.draw(&mut texture_drawer, &self.texture_manager.cheese);
                }
                self.character
                    .draw(&mut texture_drawer, &self.texture_manager.mouse);
                for spider in &mut self.curr_level.spiders {
                    spider.draw(&mut texture_drawer, &self.texture_manager.spider);
                }
                self.total_points.draw(&mut texture_drawer);
                if (self.curr_level.spiders.len() == 0) {
                    self.extra_cheese
                        .draw(&mut texture_drawer, &self.texture_manager.ceasy)
                }
            }
            Gamestate::MainMenu => {
                texture_drawer.draw_texture_pro(
                    &self.texture_manager.title,
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: 64.0,
                        height: -32.0,
                    },
                    Rectangle {
                        x: 0.0,
                        y: 32.0,
                        width: 64.0,
                        height: 32.0,
                    },
                    Vector2 { x: 0.0, y: 0.0 },
                    0.0,
                    Color::WHITE,
                );
                self.start_button
                    .draw(&mut texture_drawer, &self.texture_manager.start);
                self.option_button
                    .draw(&mut texture_drawer, &self.texture_manager.options);
            }
            Gamestate::GameOver => {
                texture_drawer.draw_texture_pro(
                    &self.texture_manager.final_score,
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: 64.0,
                        height: -11.0,
                    },
                    Rectangle {
                        x: 0.0,
                        y: 52.0,
                        width: 64.0,
                        height: 11.0,
                    },
                    Vector2 { x: 0.0, y: 0.0 },
                    0.0,
                    Color::WHITE,
                );
                self.total_points.draw(&mut texture_drawer);
                self.retry_button
                    .draw(&mut texture_drawer, &self.texture_manager.retry);
            },
            Gamestate::OptionMenu => {
                texture_drawer.draw_texture_pro(
                    &self.texture_manager.volume_text,
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: 64.0,
                        height: -32.0,
                    },
                    Rectangle {
                        x: 0.0,
                        y: 32.0,
                        width: 64.0,
                        height: 32.0,
                    },
                    Vector2 { x: 0.0, y: 0.0 },
                    0.0,
                    Color::WHITE,
                );
                self.back.draw(&mut texture_drawer, &self.texture_manager.back);
                self.volume.draw(&mut texture_drawer);
            }
            _ => (),
        }
        drop(texture_drawer);
        drawer.draw_texture_pro(
            &self.screen_texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 64.0,
                height: 64.0,
            },
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 64.0 * SCALE as f32,
                height: 64.0 * SCALE as f32,
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        )
    }
    pub fn reset_game(&mut self) {
        self.enemy_spawn_rate = 0.05;
        self.cheese_spawn_rate = 0.02;
        self.character.node.node.pos.x = 10.0;
        self.character.node.node.pos.y = 10.0;
        self.curr_level = Level::generate(
            &self.character.node.pos.x,
            &self.character.node.pos.y,
            &self.enemy_spawn_rate,
            &self.cheese_spawn_rate,
            &mut self.rand_gen,
        );
        self.total_points.translate(1.0, 62.0);
        self.total_points.reset();
        self.level_count = 0;
    }
}
