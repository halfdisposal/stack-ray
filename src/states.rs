#![allow(unused)]

use std::io::Error;
use std::io::Write;

use raylib::{ffi::CSSPalette, prelude::*};
use rand::{RngExt, rng};
use chrono::Local;


pub enum Axis {
    XAxis,
    ZAxis
}

pub enum Direction {
    Forward,
    Backward
}

pub enum GameState {
    // Main Menu State
    Main,
    
    // Load Save State
    Save,
    Load,

    // Game Init State
    Init,
    
    // Playing State
    Ready,
    Update,
    Playing,

    // Game Over State
    GameOver,

    // Pause State
    Pause,

    // Settings State
    Settings,
}

pub struct Movement {
    pub speed: f32,
    pub direction: Direction,
    pub axis: Axis
}

pub struct UserData {
    pub time: String,
    pub high_score: i32
}


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

pub struct Game {
    pub blocks: Vec<Block>,
    pub fallen_blocks: Vec<FallenBlock>,
    pub prev_block_x: Option<f32>,
    pub prev_block_z: Option<f32>,
    pub current_block: Option<Block>,
    pub state: GameState,
    pub over: bool,
    pub score: i32,
    pub high_score: i32
}

