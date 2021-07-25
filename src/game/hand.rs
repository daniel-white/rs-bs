use super::card::*;
use std::collections::BTreeSet;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Hand {
    cards: BTreeSet<Card>,
}

impl Hand {
    pub fn count(&self) -> usize {
        self.cards.len()
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    pub fn add(&mut self, card: Card) {
        self.cards.insert(card);
    }

    pub fn remove(&mut self, card: &Card) -> Option<Card> {
        self.cards.take(card)
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            cards: BTreeSet::default(),
        }
    }
}
