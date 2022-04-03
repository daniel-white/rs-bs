use crate::game::cards::{Card, Rank};
use crate::game::player::Player;
use crate::CardSet;

pub mod cards;
pub mod player;

pub type Turn = ([Option<Card>; 4], bool);

#[derive(Debug)]
pub struct GameState {
    players: Vec<Player>,
    last_turn: Option<Turn>,
    current_player: Option<usize>,
    current_rank: Rank,
    pile: CardSet,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: vec![],
            last_turn: None,
            current_player: None,
            current_rank: Rank::Ace,
            pile: CardSet::new_empty(),
        }
    }

    fn clear(&mut self) {
        self.current_rank = Rank::Ace;
        self.last_turn = None;
        self.current_player = None;
        self.pile.clear();

        self.players
            .iter_mut()
            .for_each(|player| player.clear_hand());
    }

    pub fn start_game(&mut self) {
        self.clear();

        let mut deck = CardSet::new_standard_deck();
        deck.shuffle();
        let mut i = 0;
        while let Some(card) = deck.take_top() {
            self.players[i].add_card(card);
            i = (i + 1) % self.players.len();
        }

        self.players
            .iter_mut()
            .for_each(|player| player.sort_hand());

        self.current_player = Some(0);
    }
}
