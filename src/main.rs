#![feature(bool_to_option)]
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod game;
use crate::game::*;

fn main() {
    create_world();

    //  println!("{:?}", d);
    // let mut game = Game::new();
    // game.add_player();
    // game.add_player();
    // game.start();
    //
    // game.try_turn(
    //     0,
    //     [
    //         Some(Card {
    //             suit: Suit::Hearts,
    //             rank: Rank::Ace,
    //         }),
    //         Some(Card {
    //             suit: Suit::Hearts,
    //             rank: Rank::Two,
    //         }),
    //         None,
    //         None,
    //     ],
    // )
    // .unwrap();
    //
    // println!("{:?}", game.pile);
    //
    // game.challenge(1).unwrap();
    // for player in &game.players {
    //     println!("{} {:?}", player.id, player.hand.count());
    // }
    //
    // game.try_turn(
    //     1,
    //     [
    //         Some(Card {
    //             suit: Suit::Hearts,
    //             rank: Rank::Ace,
    //         }),
    //         None,
    //         None,
    //         None,
    //     ],
    // )
    // .unwrap();

    //.unwrap();
}
