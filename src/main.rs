use std::collections::VecDeque;

use ggez::event::{self, KeyCode};
use ggez::graphics::{self, Color, DrawParam, Mesh, Rect};
use ggez::timer::delta;
use ggez::{Context, GameResult};
use rand::prelude::*;

/// Represents a vector value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Vector {
    x: usize,
    y: usize,
}

/// Represents a direction of movement.
#[derive(Copy, Clone, PartialEq)]
enum Direction {
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
enum GameState {
    /// The game is in progress.
    Running,

    /// The game has been won.
    Won,

    /// The game has been lost.
    Lost,
}

/// Represents a tile on the grid.
#[derive(Clone, Copy)]
struct Tile {
    /// The position of the tile.
    position: Vector,

    /// Whether the tile is occupied by something.
    is_occupied: bool,
}

/// Used for keeping track of the game's state.
struct MainState {
    /// The position of the snake's head.
    head_position: Vector,

    /// The positions of the snake's tail parts.
    tail_positions: VecDeque<Vector>,

    /// The dimensions of the grid.
    dimensions: Vector,

    /// The grid tiles.
    tiles: Vec<Vec<Tile>>,

    /// The position of the fruit.
    fruit_position: Vector,

    /// The direction of movement of the snake.
    movement_direction: Direction,

    /// The next direction of movement of the snake.
    queued_direction: Option<Direction>,

    /// The current state of the game.
    state: GameState,

    /// The time elapsed since the last update.
    ms_since_last_update: usize,
}

const MILLISECONDS_PER_FRAME: usize = 300;

impl MainState {
    fn new(dimensions: Vector) -> GameResult<Self> {
        let mut rng = thread_rng();

        let head_initial_position = Vector {
            x: dimensions.x / 2,
            y: dimensions.y / 2,
        };

        let mut column: Vec<Vec<Tile>> = Vec::with_capacity(dimensions.y);
        for y in 0..dimensions.y {
            let mut row = Vec::with_capacity(dimensions.x);

            for x in 0..dimensions.x {
                row.push(Tile {
                    position: Vector { x, y },
                    is_occupied: false,
                })
            }

            column.push(row);
        }

        Ok(Self {
            head_position: head_initial_position,
            tail_positions: VecDeque::new(),
            dimensions: dimensions.clone(),
            fruit_position: {
                loop {
                    let position = Vector {
                        x: rng.gen_range(0..dimensions.x),
                        y: rng.gen_range(0..dimensions.y),
                    };

                    if position == head_initial_position {
                        continue;
                    }

                    column[position.x][position.y].is_occupied = true;

                    break position;
                }
            },
            tiles: column,
            movement_direction: Direction::Right,
            queued_direction: None,
            state: GameState::Running,
            ms_since_last_update: MILLISECONDS_PER_FRAME,
        })
    }

    fn place_fruit(&mut self) -> Option<&Vector> {
        let mut unoccupied_tiles: Vec<&mut Tile> = self
            .tiles
            .iter_mut()
            .map(|tiles| {
                tiles
                    .iter_mut()
                    .filter(|tile| !tile.is_occupied)
                    .collect::<Vec<&mut Tile>>()
            })
            .flatten()
            .collect();

        if unoccupied_tiles.is_empty() {
            return None;
        }

        let mut rng = thread_rng();

        let random_index = rng.gen_range(0..unoccupied_tiles.len());
        let mut tile = unoccupied_tiles.swap_remove(random_index);

        self.fruit_position = tile.position.clone();
        tile.is_occupied = true;

        Some(&tile.position)
    }
}

const BACKGROUND_COLOR: Color = Color {
    r: 41.0 / 255.0,
    g: 41.0 / 255.0,
    b: 41.0 / 255.0,
    a: 1.0,
};
const TILE_COLORS: (Color, Color) = (
    Color {
        r: 51.0 / 255.0,
        g: 51.0 / 255.0,
        b: 51.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 59.0 / 255.0,
        g: 59.0 / 255.0,
        b: 59.0 / 255.0,
        a: 1.0,
    },
);
const HEAD_COLOR: Color = TAIL_COLORS.1;
const TAIL_COLORS: (Color, Color) = (
    Color {
        r: 12.0 / 255.0,
        g: 185.0 / 255.0,
        b: 45.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 19.0 / 255.0,
        g: 138.0 / 255.0,
        b: 54.0 / 255.0,
        a: 1.0,
    },
);
const FRUIT_COLOR: Color = Color {
    r: 255.0 / 255.0,
    g: 87.0 / 255.0,
    b: 51.0 / 255.0,
    a: 1.0,
};

const TILE_SIZE: f32 = 50.0;
const BORDER_SIZE: f32 = 10.0;

impl event::EventHandler for MainState {
    fn key_down_event(
        &mut self,
        _: &mut Context,
        keycode: event::KeyCode,
        _: event::KeyMods,
        _: bool,
    ) {
        match keycode {
            KeyCode::Up => {
                if self.movement_direction == Direction::Down {
                    return;
                }

                self.queued_direction = Some(Direction::Up);
            }
            KeyCode::Down => {
                if self.movement_direction == Direction::Up {
                    return;
                }

                self.queued_direction = Some(Direction::Down);
            }
            KeyCode::Left => {
                if self.movement_direction == Direction::Right {
                    return;
                }

                self.queued_direction = Some(Direction::Left);
            }
            KeyCode::Right => {
                if self.movement_direction == Direction::Left {
                    return;
                }

                self.queued_direction = Some(Direction::Right);
            }
            _ => (),
        }
    }

    fn update(&mut self, context: &mut Context) -> GameResult {
        if self.state == GameState::Lost {
            return Ok(());
        }

        self.ms_since_last_update += delta(context).as_millis() as usize;
        if self.ms_since_last_update < MILLISECONDS_PER_FRAME {
            return Ok(());
        }
        self.ms_since_last_update -= MILLISECONDS_PER_FRAME;

        let previous_position = self.head_position.clone();

        match self.queued_direction {
            Some(direction) => {
                self.movement_direction = direction.clone();
                self.queued_direction = None
            }
            None => (),
        }

        match self.movement_direction {
            Direction::Up => {
                if self.head_position.y == 0 {
                    self.state = GameState::Lost;
                    return Ok(());
                }

                self.head_position.y -= 1;
            }
            Direction::Down => {
                if self.head_position.y + 1 == self.dimensions.y {
                    self.state = GameState::Lost;
                    return Ok(());
                }

                self.head_position.y += 1;
            }
            Direction::Right => {
                if self.head_position.x + 1 == self.dimensions.x {
                    self.state = GameState::Lost;
                    return Ok(());
                }

                self.head_position.x += 1;
            }
            Direction::Left => {
                if self.head_position.x == 0 {
                    self.state = GameState::Lost;
                    return Ok(());
                }

                self.head_position.x -= 1;
            }
        }

        self.tiles[self.head_position.x][self.head_position.y].is_occupied = true;

        if self.tail_positions.contains(&self.head_position) {
            self.state = GameState::Lost;
        }

        self.tail_positions.push_front(previous_position);

        if self.head_position == self.fruit_position {
            // No position means a fruit could not be placed, which in turn means that
            // there are no more unoccupied tiles.
            match self.place_fruit() {
                Some(position) => self.fruit_position = position.clone(),
                None => self.state = GameState::Won,
            }

            return Ok(());
        }

        if let Some(tail_position) = self.tail_positions.pop_back() {
            self.tiles[tail_position.x][tail_position.y].is_occupied = false;
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BACKGROUND_COLOR);

        for i in 0..self.dimensions.x {
            for j in 0..self.dimensions.y {
                let is_even = (j * 10 + i) % 2 == (j % 2);

                draw_tile(
                    context,
                    &Vector { x: i, y: j },
                    if is_even {
                        TILE_COLORS.0
                    } else {
                        TILE_COLORS.1
                    },
                    1.0,
                )?;
            }
        }

        draw_tile(context, &self.head_position, HEAD_COLOR, 0.7)?;
        draw_tile(context, &self.fruit_position, FRUIT_COLOR, 0.4)?;

        for (index, tail_piece) in self.tail_positions.iter().enumerate() {
            let is_even = index % 2 == 0;

            draw_tile(
                context,
                tail_piece,
                if is_even {
                    TAIL_COLORS.0
                } else {
                    TAIL_COLORS.1
                },
                0.5,
            )?;
        }

        graphics::present(context)?;
        Ok(())
    }
}

fn draw_tile(context: &mut Context, position: &Vector, color: Color, size: f32) -> GameResult {
    let tile_size = size * TILE_SIZE;
    let padding_size = TILE_SIZE - tile_size;

    let tile = Mesh::new_rectangle(
        context,
        graphics::DrawMode::fill(),
        Rect {
            x: padding_size / 2.0 + BORDER_SIZE + (position.x as f32) * TILE_SIZE,
            y: padding_size / 2.0 + BORDER_SIZE + (position.y as f32) * TILE_SIZE,
            w: tile_size,
            h: tile_size,
        },
        color,
    )?;

    graphics::draw(context, &tile, DrawParam::default())
}

fn main() -> GameResult {
    let builder = ggez::ContextBuilder::new("snake_game", "vxern");
    let (context, event_loop) = builder.build()?;

    graphics::set_window_title(&context, "Snake Game");

    let state = MainState::new(Vector { x: 10, y: 10 })?;

    event::run(context, event_loop, state)
}
