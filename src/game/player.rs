use crate::game::cards::{Card, CardSet};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq)]
pub struct Player {
    hand: CardSet,
}

impl Player {
    pub fn new() -> Self {
        Self {
            hand: CardSet::new_empty(),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.add(card);
    }

    pub fn add_cards(&mut self, cards: &[Card]) {
        self.hand.add_all(cards);
    }

    pub fn remove_card(&mut self, card: &Card) -> Option<Card> {
        self.hand.remove(card)
    }

    pub fn sort_hand(&mut self) {
        self.hand.sort();
    }

    pub fn clear_hand(&mut self) {
        self.hand.clear();
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "hand: {}", self.hand)
    }
}
