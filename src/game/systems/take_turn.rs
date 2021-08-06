use hecs::{Entity, World};

use crate::game::cards::{Card, CardSet};
use crate::game::players::{Player, PlayerFn, PlayerQuery};
use crate::game::state::GameState;
use crate::game::Turn;
use std::collections::HashSet;
use std::ops::Index;

pub fn take_turn(world: &mut World, game_state: &mut GameState, turn: &Turn) -> Result<(), ()> {
    let cards: HashSet<_> = turn.iter().flatten().collect();
    if cards.is_empty() {
        return Err(());
    }

    let cards: Vec<_> = cards.into_iter().collect();

    let player = game_state.current_player.ok_or(())?;
    let hand = world.get_player_hand_mut(player).ok_or(())?;

    if cards.iter().all(|c| hand.contains(c)) {
        return Err(());
    }

    let truthful = cards.iter().all(|c| c.rank == game_state.current_rank);
    game_state.last_turn = Some((player, truthful));

    game_state.pile.add_all(cards.as_slice());
    hand.remove_all(cards.as_slice());

    let player_index = game_state
        .players
        .iter()
        .position(|&p| p == player)
        .unwrap();
    game_state.current_player = Some(
        *game_state
            .players
            .index((player_index + 1) % game_state.players.len()),
    );

    game_state.current_rank = game_state.current_rank.inc();

    Ok(())
}
