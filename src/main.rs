#![allow(unused)]
use std::thread::current;

use raylib::{ffi::CSSPalette, prelude::*};
use rand::{RngExt, rng};


enum Axis {
    XAxis,
    ZAxis
}

enum Direction {
    Forward,
    Backward
}

enum GameState {
    Ready,
    Update,
    Playing,
    GameOver,
    Off
}

struct Movement {
    speed: f32,
    direction: Direction,
    axis: Axis
}

struct Block {
    position: Vector3,
    size: Vector3,
    color: Color,
    movement: Movement,
}

struct FallenBlock {
    position: Vector3,
    size: Vector3,
    color: Color,
    velocity: Vector3
}

struct Game {
    blocks: Vec<Block>,
    fallen_blocks: Vec<FallenBlock>,
    prev_block_x: Option<f32>,
    prev_block_z: Option<f32>,
    current_block: Option<Block>,
    state: GameState,
    over: bool
}

impl Game {
    fn new() -> Game {
        Game{ blocks: Vec::new(), fallen_blocks: Vec::new(), prev_block_x: None, prev_block_z: None, current_block: None, state: GameState::Off, over: false }
    }

    fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    fn draw(&mut self, d3: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>, thread: &RaylibThread) {
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

}


fn move_block(block: &mut Block, base_x: f32, base_z: f32, t: f32) {
    let mut dir = 1.0;
    match block.movement.direction {
        Direction::Forward => dir = 1.0,
        Direction::Backward => dir = -1.0
    }

    match block.movement.axis {
        Axis::XAxis => {
            block.position.x = base_x + dir * (block.movement.speed * t).sin() * 5.0;
        },
        Axis::ZAxis => {
            block.position.z = base_z + dir * (block.movement.speed * t).sin() * 5.0;
        }
    }
}

fn update_fallen_blocks(game: &mut Game, dt: f32) {
    let gravity = -20.0;
    for fallen_block in game.fallen_blocks.iter_mut() {
        fallen_block.velocity.y += gravity * dt;
        fallen_block.position.y += fallen_block.velocity.y * dt;
    }
    game.fallen_blocks.retain(|fb| fb.position.y > -50.0);
}

fn trim_blocks(current: &mut Block, prev: &Block) -> Option<FallenBlock> {
    match current.movement.axis {
        Axis::XAxis => {
            let current_min = current.position.x - current.size.x / 2.0;
            let current_max = current.position.x + current.size.x / 2.0;
            let previous_min = prev.position.x - prev.size.x / 2.0;
            let previous_max = prev.position.x + prev.size.x / 2.0;

            let overlap_min = current_min.max(previous_min);
            let overlap_max = current_max.min(previous_max);
            let overlap = overlap_max - overlap_min;

            if overlap <= 0.0 {
                return None;
            }
            let cut_size = current.size.x - overlap;
            let cut_x = if current_min < previous_min {
                current_min + cut_size / 2.0
            } else {
                overlap_max + cut_size / 2.0
            };

            let fallen = FallenBlock{
                position: Vector3::new(cut_x, current.position.y, current.position.z),
                size: Vector3::new(cut_size, current.size.y, current.size.z),
                color: current.color,
                velocity: Vector3::new(rng().random_range(-3.0..3.0), 0.0, rng().random_range(-3.0..3.0))
            };
            
            current.size.x = overlap;
            current.position.x = overlap_min + overlap / 2.0;
            Some(fallen)
        },
        Axis::ZAxis => {
            let current_min = current.position.z - current.size.z / 2.0;
            let current_max = current.position.z + current.size.z / 2.0;
            let previous_min = prev.position.z - prev.size.z / 2.0;
            let previous_max = prev.position.z + prev.size.z / 2.0;

            let overlap_min = current_min.max(previous_min);
            let overlap_max = current_max.min(previous_max);
            let overlap = overlap_max - overlap_min;

            if overlap <= 0.0 {
                return None;
            }
            
            let cut_size = current.size.z - overlap;
            let cut_z = if current_min < previous_min {
                current_min + cut_size / 2.0
            } else {
                overlap_max + cut_size / 2.0
            };

            let fallen = FallenBlock{
                position: Vector3::new(current.position.x, current.position.y, cut_z),
                size: Vector3::new(current.size.x, current.size.y, cut_size),
                color: current.color,
                velocity: Vector3::new(rng().random_range(-3.0..3.0), 0.0, rng().random_range(-3.0..3.0))
            };
            current.size.z = overlap;
            current.position.z = overlap_min + overlap / 2.0;
            Some(fallen)
        }
    }
}


fn update_game_state(rl: &mut RaylibHandle, game: &mut Game, t: &mut f32) {
    let input_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE) || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);


    match game.state {
        GameState::Off => {
            let default_block = Block { 
                position: Vector3::new(0.0, 0.0, 0.0), 
                size: Vector3::new(10.0, 2.0, 10.0), 
                color: Color::BLUEVIOLET,
                movement: Movement { speed: 0.0, direction: Direction::Forward, axis: Axis::XAxis }
            };

            game.push(default_block);
            game.state = GameState::Ready;
        },
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
        GameState::Update => {
            if let Some(ref mut block) = game.current_block {
                move_block(block, game.prev_block_x.unwrap_or(0.0), game.prev_block_z.unwrap_or(0.0), *t);
                if input_pressed {
                    game.state = GameState::Playing;
                }
            }
        },
        GameState::Playing => {
            if let Some(mut block) = game.current_block.take() {
                // Check if the current block can be pushed else move to game over
                let prev = game.blocks.last().unwrap();
                match trim_blocks(&mut block, prev) {
                    None => {
                        game.state = GameState::GameOver;
                    },
                    Some(fallen) => {
                        game.fallen_blocks.push(fallen);
                        game.push(block);
                        game.current_block = None;
                        game.state = GameState::Ready;
                    }
                }
            }

        },
        GameState::GameOver => {
            game.over = true;
        }
    }

}

fn update_camera_state(game: &Game, camera: &mut Camera3D, angle: f32, dt: f32, rotate: bool) {
    let n_blocks = game.blocks.len() as f32;
    let target_y = 50.0 + 2.0 * n_blocks;
    let target_look_y = 2.0 * n_blocks;

    let speed = 5.0; // tweak this
    camera.position.y += (target_y - camera.position.y) * speed * dt;
    camera.target.y += (target_look_y - camera.target.y) * speed * dt;
    
    if rotate {
        let r = 70.0;
            
        camera.position.x = r * angle.sin();
        camera.position.z = r * angle.cos();
    }
}

fn safe_u8_add(a: u8, b: i32) -> u8 {
    let x = a as i32;
    let y = b;

    let m = 255;
    let sum = (x + y).rem_euclid(m * 2);
    let b_sum = m - (sum - m).abs();
    b_sum as u8
}

pub fn main() {
    // Constants
    let screen_width = 500;
    let screen_height = 900;
    let bg_color: Color = Color::new(210, 200, 190, 255);
    

    // Drawing Thread
    let mut game = Game::new();


    let (mut rl, thread) = raylib::init().size(screen_width, screen_height).title("Tile Breaker").build();
    let mut camera_3d = Camera3D::orthographic(Vector3::new(50.0, 50.0, 50.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 60.0);
    let mut shader = rl.load_shader(&thread, Some("assets/lighting.vert"), Some("assets/lighting.frag"));

    let ambient_loc = shader.get_shader_location("ambient");
    shader.set_shader_value(ambient_loc, Vector4::new(0.2, 0.2, 0.2, 1.0));
    
    let light_pos_loc = shader.get_shader_location("lightPos");
    shader.set_shader_value(light_pos_loc, Vector3::new(0.0, 50.0, 0.0));

    let view_pos_loc = shader.get_shader_location("viewPos");

    let mut image = Image::gen_image_color(screen_width, screen_height, Color::new(200, 200, 200, 100));
    image.blur_gaussian(9);
    image.draw_text("GAME OVER", (screen_width.as_f32() * 0.2) as i32, screen_height / 2, 48, Color::BLACK);
    let texture = rl.load_texture_from_image(&thread, &image).unwrap();

    rl.set_target_fps(60);
    let mut t = 0.0;
    let mut dt = 0.0;
    let mut angle = 0.0f32;

    while !rl.window_should_close() {
        dt = rl.get_frame_time();
        t += dt;
        angle += 0.15 * dt;
        if (angle - 2.0 * PI as f32).abs() < 1e-3 {
            angle = 0.0f32;
        }


        update_game_state(&mut rl, &mut game, &mut t);
        update_camera_state(&game, &mut camera_3d, angle, dt, true);
        shader.set_shader_value(view_pos_loc, camera_3d.position);
        update_fallen_blocks(&mut game, dt);


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg_color);

        {
            let mut d3 = d.begin_mode3D(camera_3d);
            {
                let mut ds = d3.begin_shader_mode(&mut shader);
                game.draw(&mut ds, &thread);
            }
            // d3.draw_grid(10, 1.0);
        }

        d.draw_text(&format!("{}", game.blocks.len())[..], screen_width / 2 - 1, 50, 48, Color::BLACK);
        if game.over {
            d.draw_texture(&texture, 0, 0, Color::WHITE);
        }
        d.draw_fps(10, 10);
    }
}
