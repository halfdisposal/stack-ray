use raylib::prelude::*;

use crate::game::Game;

pub fn update_camera_state(game: &Game, camera: &mut Camera3D, angle: f32, dt: f32, rotate: bool) {
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
