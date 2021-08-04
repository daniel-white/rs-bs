use crate::game::cards::CardSet;
use specs::prelude::*;
use specs::{Builder, Component, Entity, World, WorldExt};

#[derive(Component)]
pub struct Player;

pub trait CreatePlayer {
    fn create_player(&mut self) -> Entity;
}

impl CreatePlayer for World {
    fn create_player(&mut self) -> Entity {
        self.create_entity()
            .with(Player)
            .with(CardSet::default())
            .build()
    }
}
