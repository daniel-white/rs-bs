// use crate::game::cards::{Card, CardSet};
// use crate::game::players::{Player, PlayerFn, PlayerQuery};
// use crate::game::state::GameState;
// use crate::game::state::Turn;
// use std::collections::HashSet;
// use std::ops::Index;
//
// pub fn call_bs(game_state: &mut GameState, challenging_player: Entity) -> Result<(), ()> {
//     if game_state.pile.is_empty() {
//         return Err(());
//     }
//
//     let (last_player, was_truthful) = game_state.last_turn.ok_or(())?;
//
//     if last_player == challenging_player {
//         return Err(());
//     }
//
//     let unlucky_player = if was_truthful {
//         challenging_player
//     } else {
//         last_player
//     };
//
//     //let mut hand = world.get_player_hand_mut(unlucky_player).ok_or(())?;
//
//     while let Some(card) = game_state.pile.take_top() {
//         hand.add(card);
//     }
//
//     hand.sort_by_rank();
//
//     Ok(())
// }
