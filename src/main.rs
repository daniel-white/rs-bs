#![feature(bool_to_option)]
#![feature(in_band_lifetimes)]
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod game;

use crate::game::cards::CardSet;

fn main() {
    let mut cards = CardSet::new_standard_deck();
    println!("Pre: {}", cards);
    cards.shuffle();
    println!("Post: {}", cards);
    cards.sort();
    println!("Post: {}", cards);
}
