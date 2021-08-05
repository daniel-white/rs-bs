use super::cards::{CardSet, Rank};
use hecs::Entity;

#[derive(Debug)]
pub struct GameState {
    pub players: Vec<Entity>,
    pub last_turn: Option<(Entity, bool)>,
    pub current_player: Option<Entity>,
    pub current_rank: Rank,
    pub pile: CardSet,
}
