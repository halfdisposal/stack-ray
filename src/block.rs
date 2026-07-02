use raylib::prelude::*;
use rand::{RngExt, rng};

use crate::types::{Axis, Direction, Movement};

pub struct Block {
    pub position: Vector3,
    pub size: Vector3,
    pub color: Color,
    pub movement: Movement,
}

pub struct FallenBlock {
    pub position: Vector3,
    pub size: Vector3,
    pub color: Color,
    pub velocity: Vector3
}

pub fn move_block(block: &mut Block, base_x: f32, base_z: f32, t: f32) {
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

pub fn update_fallen_blocks(fallen_blocks: &mut Vec<FallenBlock>, dt: f32) {
    let gravity = -20.0;
    for fallen_block in fallen_blocks.iter_mut() {
        fallen_block.velocity.y += gravity * dt;
        fallen_block.position.y += fallen_block.velocity.y * dt;
    }
    fallen_blocks.retain(|fb| fb.position.y > -50.0);
}

pub fn trim_blocks(current: &mut Block, prev: &Block) -> Option<FallenBlock> {
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
