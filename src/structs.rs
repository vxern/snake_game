/// Represents a vector value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector {
    pub x: usize,
    pub y: usize,
}

/// Represents a direction of movement.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    /// Upwards.
    Up,

    /// Downwards.
    Down,

    /// Rightwards.
    Right,

    /// Leftwards
    Left,
}

/// Represents the state of the current game.
#[derive(Debug, PartialEq)]
pub enum State {
    /// The game is in progress.
    Running,

    /// The game has been won.
    Won,

    /// The game has been lost.
    Lost,
}

/// Represents a tile on the grid.
#[derive(Clone, Copy)]
pub struct Tile {
    /// The position of the tile.
    pub position: Vector,

    /// Whether the tile is occupied by something.
    pub is_occupied: bool,
}
