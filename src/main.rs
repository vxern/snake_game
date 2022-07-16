mod constants;
mod game;
mod structs;

use game::GameState;
use structs::Vector;

use ggez::{event, graphics, GameResult};

fn main() -> GameResult {
    let builder = ggez::ContextBuilder::new("snake_game", "vxern");
    let (context, event_loop) = builder.build()?;

    graphics::set_window_title(&context, "Snake Game");

    let state = GameState::new(Vector { x: 10, y: 10 })?;

    event::run(context, event_loop, state)
}
