mod usr;
mod eng;
#[macro_use]
extern crate glium;

use eng::core::game::Game;

pub fn main() {
   Game::new().run();
}