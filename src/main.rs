#![feature(bool_to_option)]
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod game;
use crate::game::*;

fn main() {
    create_world();
}
