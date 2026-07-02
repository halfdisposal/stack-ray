use raylib::ffi::Texture;
use raylib::prelude::*;
use rand::{RngExt, rng};

use crate::types::{Axis, Direction, GameState, Movement};
use crate::block::{Block, FallenBlock, move_block, trim_blocks};
use crate::user_data::UserData;
use crate::utils::{safe_u8_add, Settings};
use crate::ui::{Button, ButtonState, ToggleButton};

pub struct Resource {
    pub main_menu_bg: Option<Texture2D>,
    pub blur_texture: Option<Texture2D>,
}

impl Resource {
    pub fn new() -> Self {
        Resource { 
            main_menu_bg: None, 
            blur_texture: None 
        }
    }

    pub fn reset(&mut self) {
        self.blur_texture = None;
    }
}

pub struct Game {
    pub running: bool,
    pub blocks: Vec<Block>,
    pub fallen_blocks: Vec<FallenBlock>,
    pub prev_block_x: Option<f32>,
    pub prev_block_z: Option<f32>,
    pub current_block: Option<Block>,
    pub state: GameState,
    pub over: bool,
    pub score: i32,
    pub high_score: i32,
    pub buttons: Option<Vec<Button>>,
    pub toggle_buttons: Option<Vec<ToggleButton>>,
    pub settings: Settings,
    pub resources: Resource
}

impl Game {
    pub fn new() -> Game {
        Game{ running: true, blocks: Vec::new(), fallen_blocks: Vec::new(), prev_block_x: None, prev_block_z: None, current_block: None, state: GameState::Main, over: false, score: 0, high_score: 0, buttons: None, toggle_buttons: None, settings: Settings::new(), resources: Resource::new() }
    }

    pub fn reset(&mut self) {
        self.blocks = Vec::new();
        self.fallen_blocks = Vec::new();
        self.prev_block_x = None;
        self.prev_block_z = None;
        self.current_block = None;
        self.state = GameState::Init;
        self.over = false;
        self.score = 0;
        self.buttons = None;
        self.toggle_buttons = None;
        self.resources.reset();
    }

    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn draw_blocks(&mut self, d3: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>, thread: &RaylibThread) {
        for (i, block) in self.blocks.iter().enumerate() {
            d3.draw_cube(block.position, block.size.x, block.size.y, block.size.z, block.color);
            let mut c = block.color;
            c.r = (c.r.as_f32() * 0.3) as u8;
            c.g = (c.g.as_f32() * 0.3) as u8;
            c.b = (c.b.as_f32() * 0.3) as u8;
            d3.draw_cube_wires(block.position, block.size.x, block.size.y, block.size.z, c);
        }
        if let Some(ref block) = self.current_block {
            d3.draw_cube(block.position, block.size.x, block.size.y, block.size.z, block.color);
            let mut c = block.color;
            c.r = (c.r.as_f32() * 0.3) as u8;
            c.g = (c.g.as_f32() * 0.3) as u8;
            c.b = (c.b.as_f32() * 0.3) as u8;
            d3.draw_cube_wires(block.position, block.size.x, block.size.y, block.size.z, c);
        }
        for (i, fallen_block) in self.fallen_blocks.iter().enumerate() {
            d3.draw_cube(fallen_block.position, fallen_block.size.x, fallen_block.size.y, fallen_block.size.z, fallen_block.color);
            let mut c = fallen_block.color;
            c.r = (c.r.as_f32() * 0.3) as u8;
            c.g = (c.g.as_f32() * 0.3) as u8;
            c.b = (c.b.as_f32() * 0.3) as u8;
            d3.draw_cube_wires(fallen_block.position, fallen_block.size.x, fallen_block.size.y, fallen_block.size.z, c);
        }
    }

    pub fn draw3d(&mut self, d3: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>, thread: &RaylibThread) {
        self.draw_blocks(d3, thread);
    }


    pub fn draw_buttons(&mut self, d: &mut RaylibDrawHandle<'_>, thread: &RaylibThread) {
        if let Some(ref buttons) = self.buttons {
            for button in buttons {
                button.draw(d);
            }
        }
        if let Some(ref buttons) = self.toggle_buttons {
            for button in buttons {
                button.draw(d);
            }
        }}

    pub fn draw2d(&mut self, d: &mut RaylibDrawHandle<'_>, thread: &RaylibThread) {
        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();
        match self.state {
            GameState::GameOver => {
                let msg = "GAME OVER";
                let score = format!("SCORE: {}", self.score);
                let high_score = format!("HIGH SCORE: {}", self.high_score);

                let msg_width = d.measure_text(msg, 48);
                let score_width = d.measure_text(&score, 36);
                let high_score_width = d.measure_text(&high_score, 36);

                d.draw_rectangle(0, 0, screen_width, screen_height, Color::new(255, 255, 255, 100));

                d.draw_text(msg, screen_width / 2 - msg_width / 2, screen_height / 2 - 130, 48, Color::BLACK);
                d.draw_text(&score, screen_width / 2 - score_width / 2, screen_height / 2 - 50, 36, Color::BLACK);
                d.draw_text(&high_score, screen_width / 2 - high_score_width / 2, screen_height / 2, 36, Color::BLACK);
                self.draw_buttons(d, thread);
            },
            GameState::Main => {
                if let Some(ref texture) = self.resources.main_menu_bg {
                    d.draw_texture(texture, 0, 0, Color::WHITE);
                }
                let msg = "TILE BREAKER";
                let msg_width = d.measure_text(msg, 48);
                d.draw_text(msg, screen_width / 2 - msg_width / 2, screen_height / 2 - 130, 48, Color::BLACK);
                self.draw_buttons(d, thread);
            },
            GameState::Pause => {
                if let Some(ref texture) = self.resources.blur_texture {
                    d.draw_texture(texture, 0, 0, Color::WHITE);
                }
                d.draw_rectangle(0, 0, screen_width, screen_height, Color::new(255, 255, 255, 100));


                let msg = "PAUSED";
                let msg_width = d.measure_text(msg, 48);                
                d.draw_text(msg, screen_width / 2 - msg_width / 2, screen_height / 2 - 130, 48, Color::BLACK);
                self.draw_buttons(d, thread);
            },
            GameState::Settings => {
                if let Some(ref texture) = self.resources.blur_texture {
                    d.draw_texture(texture, 0, 0, Color::WHITE);
                }
                d.draw_rectangle(0, 0, screen_width, screen_height, Color::new(255, 255, 255, 100));

                let msg = "SETTINGS";
                let msg_width = d.measure_text(msg, 48);
                d.draw_text(msg, screen_width / 2 - msg_width / 2, screen_height / 2 - 130, 48, Color::BLACK);
                self.draw_buttons(d, thread);
            }
            _ => {
                d.draw_text(&format!("{}", self.score), screen_width / 2 - 1, 50, 48, Color::BLACK);
                self.draw_buttons(d, thread);
            }
        }
    }

    pub fn capture_blur_background(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if self.resources.blur_texture.is_some() {
            return;
        }
 
        let mut image = rl.load_image_from_screen(thread);
        image.blur_gaussian(4);
 
        if let Ok(texture) = rl.load_texture_from_image(thread, &image) {
            self.resources.blur_texture = Some(texture);
        }
    }

    pub fn clear_blur_background(&mut self) {
        self.resources.blur_texture = None;
    }
}

pub fn update_game_state(rl: &mut RaylibHandle, thread: &RaylibThread, game: &mut Game, t: &mut f32, dt: f32, user_data_file: &String) {
    let input_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);


    match game.state {
        // Initialize by the default block
        GameState::Init => {
            let default_block = Block {
                position: Vector3::new(0.0, 0.0, 0.0),
                size: Vector3::new(10.0, 2.0, 10.0),
                color: Color::BLUEVIOLET,
                movement: Movement { speed: 0.0, direction: Direction::Forward, axis: Axis::XAxis }
            };

            game.push(default_block);
            game.state = GameState::Ready;
        },
        // Prepares the moving block
        GameState::Ready => {
            *t = 0.0;
            let i = game.blocks.len() as f32;
            let r = safe_u8_add(game.blocks[0].color.r, (i as i32) * 9);
            let g = safe_u8_add(game.blocks[0].color.g, (i as i32) * 9);
            let b = safe_u8_add(game.blocks[0].color.b, (i as i32) * 9);
            let random_dir = rng().random_range(0..=1);
            let mut direction = Direction::Forward;
            if random_dir == 0 {
                direction = Direction::Backward;
            }

            let speed = (1.0 + i * 0.1);

            let prev = game.blocks.last().unwrap();
            let axis = match prev.movement.axis { Axis::XAxis => Axis::ZAxis, Axis::ZAxis => Axis::XAxis };

            let mut block = Block {
                position: Vector3::new(0.0, 2.0 * i, 0.0),
                size: Vector3::new(prev.size.z, 2.0, prev.size.z),
                color: Color::new(r, g, b, 255),
                movement: Movement { speed, direction, axis }
            };
            game.prev_block_x = Some(prev.position.x);
            game.prev_block_z = Some(prev.position.z);

            block.position.x = prev.position.x;
            block.position.z = prev.position.z;



            game.current_block = Some(block);
            game.state = GameState::Update;
        },
        // Looping Block and Move the current block
        GameState::Update => { 
            match &mut game.buttons {
                None => {
                    let mut b = vec![
                    Button::new(
                        rl.get_screen_width() as f32 - 70.0, rl.get_screen_height() as f32 - 40.0, 90.0, 40.0, "Pause", 
                        ButtonState{ default: Color::new(197, 224, 255, 255), hover: Color::new(150, 203, 255, 255), clicked: Color::DARKGRAY }, 18
                    ),
                    ];
                    game.buttons = Some(b);
                },
                Some(buttons) => {
                    if buttons[0].is_clicked(rl) {
                        println!("INFO: Paused");
                        game.buttons = None;
                        game.state = GameState::Pause;
                        game.capture_blur_background(rl, thread);
                    }
                }
            }
            if let Some(ref mut block) = game.current_block {
                *t += dt;
                move_block(block, game.prev_block_x.unwrap_or(0.0), game.prev_block_z.unwrap_or(0.0), *t);
                if input_pressed {
                    game.state = GameState::Playing;
                } 
            } else {
                game.state = GameState::Ready;
            }
        },
        // Handle Playing when input detected
        GameState::Playing => {
            if let Some(mut block) = game.current_block.take() {
                // Check if the current block can be pushed else move to game over
                let prev = game.blocks.last().unwrap();
                match trim_blocks(&mut block, prev) {
                    None => {
                        game.buttons = None;
                        game.state = GameState::Save;
                    },
                    Some(fallen) => {
                        game.fallen_blocks.push(fallen);
                        game.push(block);
                        game.score += 1;
                        game.current_block = None;
                        game.state = GameState::Ready;
                    }
                }
            }

        },
        // Handle Game Over
        GameState::GameOver => {
            game.over = true;
            // Game Over Scene
            match &mut game.buttons {
                None => {
                    let mut b = vec![
                    Button::new(
                        rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + 120.0, 120.0, 50.0, "Main Menu", 
                        ButtonState{ default: Color::new(197, 224, 255, 255), hover: Color::new(150, 203, 255, 255), clicked: Color::DARKGRAY }, 20
                    ),
                    Button::new(
                        rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + 195.0, 120.0, 50.0, "Restart", 
                        ButtonState{ default: Color::new(197, 224, 255, 255), hover: Color::new(150, 203, 255, 255), clicked: Color::DARKGRAY }, 20
                    )
                    ];
                    game.buttons = Some(b);
                },
                Some(buttons) => {
                    if buttons[0].is_clicked(rl) {
                        println!("INFO: Main Menu");
                        game.buttons = None;
                        game.state = GameState::Main;
                    } else if buttons[1].is_clicked(rl) {
                        println!("INFO: Restart");
                        game.buttons = None;
                        game.reset();
                    }
                }
            }
        },
        // Handle Save
        GameState::Save => {
            let highscore = game.score.max(game.high_score);

            if highscore <= game.score {
                game.high_score = highscore;
                let mut user_data = UserData { time: "0.0.0".to_string(), high_score: game.score.max(game.high_score) };
                user_data.save(user_data_file);
            }


            let settings_file = "./data/settings.txt".to_string();
            game.settings.save(&settings_file);
            game.state = GameState::GameOver;
        },
        // Handle Load
        GameState::Load => {
            let user_data = UserData::load(user_data_file);
            game.high_score = user_data.high_score;
            game.state = GameState::Init;
        },
        // Handle Main Menu
        GameState::Main => {
            // Main Menu Scene
            let init = 0.0;
            let sep = 75.0;
            match &mut game.buttons {
                None => {
                    let mut b = vec![
                    Button::new(
                        rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + init, 90.0, 50.0, "Play", 
                        ButtonState{ default: Color::new(197, 224, 255, 255), hover: Color::new(150, 203, 255, 255), clicked: Color::DARKGRAY }, 24
                    ),
                    Button::new(
                        rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + init + sep, 90.0, 50.0, "Load", 
                        ButtonState{ default: Color::new(197, 224, 255, 255), hover: Color::new(150, 203, 255, 255), clicked: Color::DARKGRAY }, 24
                    ),
                    Button::new(
                        rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + init + 2.0 * sep, 90.0, 50.0, "Quit", 
                        ButtonState{ default: Color::new(197, 224, 255, 255), hover: Color::new(150, 203, 255, 255), clicked: Color::DARKGRAY }, 24
                    )
                    ];
                    game.buttons = Some(b);
                },
                Some(buttons) => {
                    if buttons[0].is_clicked(rl) {
                        println!("INFO: New Game");
                        game.reset();
                        game.buttons = None;
                        game.state = GameState::Init;
                    } else if buttons[1].is_clicked(rl) {
                        println!("INFO: Load Game");
                        game.reset();
                        game.buttons = None;
                        game.state = GameState::Load;
                    } else if buttons[2].is_clicked(rl) {
                        println!("INFO: Quit");
                        game.reset();
                        game.running = false;
                    }
                }
            }
        },
        // Handle Pause
        GameState::Pause => {
            // Pause Scene 
            let init = 0.0;
            let sep = 75.0;
            match &mut game.buttons {
                None => {
                    let mut b = vec![
                    Button::new(
                        rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + init, 120.0, 50.0, "Resume", 
                        ButtonState{ default: Color::WHITE, hover: Color::GRAY, clicked: Color::DARKGRAY }, 20
                    ),
                    Button::new(
                        rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + init + sep, 120.0, 50.0, "Settings", 
                        ButtonState{ default: Color::WHITE, hover: Color::GRAY, clicked: Color::DARKGRAY }, 20
                    ),
                    Button::new(
                        rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + init + 2.0 * sep, 120.0, 50.0, "Main Menu", 
                        ButtonState{ default: Color::WHITE, hover: Color::GRAY, clicked: Color::DARKGRAY }, 20
                    ), 
                    Button::new(
                        rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + init + 3.0 * sep, 120.0, 50.0, "Quit", 
                        ButtonState{ default: Color::WHITE, hover: Color::GRAY, clicked: Color::DARKGRAY }, 20
                    )
                    ];
                    game.buttons = Some(b);
                },
                Some(buttons) => {
                    if buttons[0].is_clicked(rl) {
                        println!("INFO: Resume Game");
                        game.buttons = None;
                        game.state = GameState::Update;
                        game.clear_blur_background();
                    } else if buttons[1].is_clicked(rl) {
                        println!("INFO: Settings");
                        game.buttons = None;
                        game.state = GameState::Settings;
                    } else if buttons[2].is_clicked(rl) {
                        println!("INFO: Main Menu");
                        game.buttons = None;
                        game.state = GameState::Main;
                        game.clear_blur_background();
                    } else if buttons[3].is_clicked(rl) {
                        println!("INFO: Quit");
                        game.running = false;
                    }
                }
            }
        },
        // Handle Settings
        GameState::Settings => {
            // Settings Scene 
            let init = 0.0;
            let sep = 75.0;
            match &mut game.toggle_buttons {
                None => {
                    let mut b = vec![
                        ToggleButton::new(
                            rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + init + 2.0 * sep, 150.0, 50.0, "Rotate", 
                            ButtonState{ default: Color::WHITE, hover: Color::BLUE, clicked: Color::CYAN }, 20, 25.0, game.settings.rotation
                        ),
                    ];
                    game.toggle_buttons = Some(b);
                },
                Some(buttons) => {
                    if buttons[0].is_clicked(rl) {
                        println!("INFO: Rotation {}", buttons[0].on);
                        game.settings.rotation = buttons[0].on;
                    }
                }
            }
            match &mut game.buttons {
                None => {
                    let mut b = vec![
                    Button::new(
                        rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + init, 120.0, 50.0, "Resume", 
                        ButtonState{ default: Color::WHITE, hover: Color::GRAY, clicked: Color::DARKGRAY }, 20
                    ),
                    Button::new(
                        rl.get_screen_width() as f32/ 2.0, rl.get_screen_height() as f32 / 2.0 + init + 1.0 * sep, 120.0, 50.0, "Quit", 
                        ButtonState{ default: Color::WHITE, hover: Color::GRAY, clicked: Color::DARKGRAY }, 20
                    )
                    ];
                    game.buttons = Some(b);
                },
                Some(buttons) => {
                    if buttons[0].is_clicked(rl) {
                        println!("INFO: Resume Game");
                        game.buttons = None;
                        game.toggle_buttons = None;
                        game.state = GameState::Update;
                    } else if buttons[1].is_clicked(rl) {
                        println!("INFO: Quit");
                        game.running = false;
                    }
                }
            }
            if rl.is_key_pressed(KeyboardKey::KEY_P) {
                game.toggle_buttons = None;
                game.state = GameState::Update;
            } else if rl.is_key_pressed(KeyboardKey::KEY_R) {
                game.reset();
            }
        },
        // Other States
        _ => {}
    }

}
