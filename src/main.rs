mod card;
mod deck;
mod hand;
mod strategy;
mod stats;
mod game;

use crate::game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}

