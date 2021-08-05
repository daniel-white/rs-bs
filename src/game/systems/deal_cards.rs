use hecs::World;

use crate::game::cards::CardSet;
use crate::game::players::PlayerQuery;

pub fn deal_cards(world: &mut World) {
    let mut hands: Vec<_> = world
        .query_mut::<PlayerQuery>()
        .into_iter()
        .map(|(_, (_, hand))| hand)
        .collect();

    hands.iter_mut().for_each(|hand| hand.clear());

    let mut deck = CardSet::standard_deck();
    deck.shuffle();

    let mut i = 0;
    while let Some(card) = deck.take_top() {
        hands[i].add(card);
        i = (i + 1) % hands.len();
    }

    hands.iter_mut().for_each(|hand| hand.sort_by_rank());
}
