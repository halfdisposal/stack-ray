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
