use super::card::*;
use super::hand::Hand;
use rand::{thread_rng, Rng};
use std::collections::vec_deque::IntoIter;
use std::collections::VecDeque;
use std::ops::Range;
use strum::IntoEnumIterator;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = VecDeque::with_capacity(52);

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push_front(Card { suit, rank });
            }
        }

        Self { cards }
    }
}

impl Deck {
    pub fn shuffle(&mut self) {
        self.cards.len();
        let mut rng = thread_rng();
        let mut i = self.cards.len();
        while i >= 2 {
            i -= 1;
            self.cards.swap(
                i,
                rng.gen_range(Range {
                    start: 0,
                    end: i + 1,
                }),
            );
        }
    }

    pub fn count(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    pub fn draw_into(&mut self, hand: &mut Hand) -> Result<(), ()> {
        match self.draw() {
            Some(card) => {
                hand.add(card);
                Ok(())
            }
            None => Err(()),
        }
    }

    pub fn deal(&mut self, hands: &mut [&mut Hand]) {
        let mut count = 0;
        while let Some(card) = self.draw() {
            let hand_index = count % hands.len();
            let hand = &mut hands[hand_index];
            (*hand).add(card);
            count += 1;
        }
    }
}

impl IntoIterator for Deck {
    type Item = Card;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}
