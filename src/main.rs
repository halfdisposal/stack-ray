#![allow(unused)]

mod types;
mod block;
mod camera;
mod game;
mod user_data;
mod utils;
mod ui;

use raylib::{ffi::Texture, prelude::*};

use game::{Game, update_game_state};
use camera::update_camera_state;
use block::update_fallen_blocks;

use crate::types::GameState;

pub fn main() {
    // Constants
    let screen_width = 500;
    let screen_height = 900;
    let bg_color: Color = Color::new(210, 200, 190, 255);
    let user_data_file = "./data/user_data.txt".to_string();
    let settings_file = "./data/settings.txt".to_string();
    let main_menu_bg = "./assets/main_menu_bg.png";

    // Drawing Thread
    let mut game = Game::new();
    game.settings = utils::Settings::load(&settings_file);

    let (mut rl, thread) = raylib::init().size(screen_width, screen_height).title("Tile Breaker").msaa_4x().build();
    let mut camera_3d = Camera3D::orthographic(Vector3::new(50.0, 50.0, 50.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 60.0);
    let mut shader = rl.load_shader(&thread, Some("assets/lighting.vert"), Some("assets/lighting.frag"));

    let ambient_loc = shader.get_shader_location("ambient");
    shader.set_shader_value(ambient_loc, Vector4::new(0.2, 0.2, 0.2, 1.0));

    let light_pos_loc = shader.get_shader_location("lightPos");
    shader.set_shader_value(light_pos_loc, Vector3::new(0.0, 50.0, 0.0));

    let main_menu_bg_texture = rl.load_texture(&thread, main_menu_bg).unwrap();
    game.resources.main_menu_bg = Some(main_menu_bg_texture);

    let view_pos_loc = shader.get_shader_location("viewPos");

    rl.set_target_fps(60);
    let mut t = 0.0;
    let mut dt = 0.0;
    let mut angle = 0.0f32;

    while game.running && !rl.window_should_close() {
        dt = rl.get_frame_time();

        if game.settings.rotation {
            angle += 0.15 * dt;
            if (angle - 2.0 * PI as f32).abs() < 1e-3 {
                angle = 0.0f32;
            }
        }


        update_game_state(&mut rl, &thread, &mut game, &mut t, dt, &user_data_file);
        update_camera_state(&game, &mut camera_3d, angle, dt, game.settings.rotation);
        shader.set_shader_value(view_pos_loc, camera_3d.position);
        update_fallen_blocks(&mut game.fallen_blocks, dt);



        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg_color);

        {
            let mut d3 = d.begin_mode3D(camera_3d);
            {
                let mut ds = d3.begin_shader_mode(&mut shader);
                game.draw3d(&mut ds, &thread);
            }
            // d3.draw_grid(10, 1.0);
        }
        game.draw2d(&mut d, &thread);

        d.draw_fps(10, 10);
    }
}
