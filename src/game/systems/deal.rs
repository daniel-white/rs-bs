use crate::game::cards::CardSet;
use specs::prelude::*;
use std::collections::HashSet;

pub struct Deal {
    players: HashSet<Entity>,
}

impl Deal {
    pub fn new() -> Self {
        Deal {
            players: HashSet::default(),
        }
    }

    pub fn with_players(&mut self, players: &[Entity]) -> &mut Self {
        self.players.clear();
        for player in players {
            self.players.insert(*player);
        }
        self
    }
}

impl<'a> System<'a> for Deal {
    type SystemData = WriteStorage<'a, CardSet>;

    fn run(&mut self, mut cards: Self::SystemData) {
        for player in self.players.iter() {
            let player_cards = cards.get_mut(*player).unwrap();
            player_cards.clear();
        }

        let mut deck = CardSet::standard_deck();
        deck.shuffle();

        for player in self.players.iter().cycle() {
            let player_cards = cards.get_mut(*player).unwrap();
            if let Some(card) = deck.take_top() {
                player_cards.add(card);
            } else {
                break;
            }
        }

        for player in self.players.iter() {
            let player_cards = cards.get_mut(*player).unwrap();
            player_cards.sort_by_rank();
        }
    }
}
