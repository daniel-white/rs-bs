#![feature(bool_to_option)]

mod game;
use crate::game::card::*;
use crate::game::deck::Deck;
use crate::game::hand::Hand;

fn main() {
    let mut deck = Deck::default();
    deck.shuffle();

    let mut hand1 = Hand::default();
    let mut hand2 = Hand::default();
    let mut hand3 = Hand::default();
    let mut hand4 = Hand::default();

    let hands = &mut vec![&mut hand1, &mut hand2, &mut hand3, &mut hand4];

    deck.deal(hands);

    println!("{:?}", deck);

    for hand in hands {
        println!("{:?}", hand);
        hand.remove(&Card {
            rank: Rank::Ace,
            suit: Suit::Hearts,
        })
        .unwrap();
        println!("{:?}", hand.count());
    }
}
