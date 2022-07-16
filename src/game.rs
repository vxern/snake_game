use crate::{constants::*, structs::*};

use std::collections::VecDeque;

use ggez::event::{self, KeyCode};
use ggez::graphics::{self, Color, DrawParam, Mesh, Rect};
use ggez::timer::delta;
use ggez::{Context, GameResult};
use rand::prelude::*;

/// Used for keeping track of the game's state.
pub struct GameState {
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
    state: State,

    /// The time elapsed since the last update.
    ms_since_last_update: usize,
}

impl GameState {
    pub fn new(dimensions: Vector) -> GameResult<Self> {
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
            state: State::Running,
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

impl event::EventHandler for GameState {
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
        if self.state == State::Lost {
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
                    self.state = State::Lost;
                    return Ok(());
                }

                self.head_position.y -= 1;
            }
            Direction::Down => {
                if self.head_position.y + 1 == self.dimensions.y {
                    self.state = State::Lost;
                    return Ok(());
                }

                self.head_position.y += 1;
            }
            Direction::Right => {
                if self.head_position.x + 1 == self.dimensions.x {
                    self.state = State::Lost;
                    return Ok(());
                }

                self.head_position.x += 1;
            }
            Direction::Left => {
                if self.head_position.x == 0 {
                    self.state = State::Lost;
                    return Ok(());
                }

                self.head_position.x -= 1;
            }
        }

        self.tiles[self.head_position.y][self.head_position.x].is_occupied = true;

        if self.tail_positions.contains(&self.head_position) {
            self.state = State::Lost;
        }

        self.tail_positions.push_front(previous_position);

        if self.head_position == self.fruit_position {
            // No position means a fruit could not be placed, which in turn means that
            // there are no more unoccupied tiles.
            match self.place_fruit() {
                Some(position) => self.fruit_position = position.clone(),
                None => self.state = State::Won,
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
