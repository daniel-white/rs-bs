extern crate strum;
#[macro_use]
extern crate strum_macros;

mod game;

use crate::game::{
    cards::{Card, Rank, Suit},
    *,
};

fn main() {
    let mut game = Game::new(4);

    game.start();
    println!("{}", game);

    game.take_turn([Some(Card::new(Rank::Ten, Suit::Spades)), None, None, None]);
    println!("{}", game);

    game.take_turn([Some(Card::new(Rank::Ace, Suit::Spades)), None, None, None]);
    println!("{}", game);

    game.call_bs(0);
    println!("{}", game);
}
