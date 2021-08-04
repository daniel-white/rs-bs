use crate::game::cards::{Card, CardSet};
use specs::prelude::*;

pub struct TakeTurn {
    player: Option<Entity>,
    cards: CardSet,
}

impl TakeTurn {
    pub fn new() -> Self {
        TakeTurn {
            player: None,
            cards: CardSet::new(),
        }
    }

    pub fn with_player(&mut self, player: Entity) -> &mut Self {
        self.player = Some(player);
        self
    }

    pub fn with_cards(&mut self, cards: &[Option<Card>; 4]) -> &mut Self {
        for Some(card) in cards {
            self.cards.add(*card);
        }
        self
    }
}

impl<'a> System<'a> for TakeTurn {
    type SystemData = WriteStorage<'a, CardSet>;

    fn run(&mut self, mut cards: Self::SystemData) {}
}
