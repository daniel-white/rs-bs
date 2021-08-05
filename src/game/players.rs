use crate::game::cards::CardSet;
use hecs::{Entity, World};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

pub type PlayerQuery<'a> = (&'a Player, &'a mut CardSet);

pub trait PlayerFn {
    fn create_player(&mut self) -> Entity;
    fn get_player_hand_mut(&mut self, player: Entity) -> Option<&mut CardSet>;
}

impl PlayerFn for World {
    fn create_player(&mut self) -> Entity {
        self.spawn((Player, CardSet::default()))
    }

    fn get_player_hand_mut(&mut self, player: Entity) -> Option<&mut CardSet> {
        match self.query_one_mut::<PlayerQuery>(player) {
            Ok((_, hand)) => Some(hand),
            _ => None,
        }
    }
}
