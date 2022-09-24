use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::game::cards::{Card, CardSet, Rank};
use crate::game::player::Player;

pub mod cards;
pub mod player;

type Turn = (usize, Vec<Card>, bool);

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    last_turn: Option<Turn>,
    current_player_index: Option<usize>,
    current_rank: Rank,
    pile: CardSet,
}

impl Game {
    pub fn new(player_count: u8) -> Self {
        Self {
            players: (0..player_count).map(|_| Player::new()).collect(),
            last_turn: None,
            current_player_index: None,
            current_rank: Rank::Ace,
            pile: CardSet::new_empty(),
        }
    }

    pub fn start(&mut self) {
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

        self.current_player_index = Some(0);
    }

    pub fn take_turn(&mut self, cards: [Option<Card>; 4]) {
        let current_player_index = self.current_player_index.expect("no current player");
        let player = self
            .players
            .get_mut(current_player_index)
            .expect("missing player");

        let cards = cards
            .iter()
            .flatten()
            .map(|card| player.remove_card(card).expect("card not in hand"))
            .collect::<Vec<_>>();

        let is_truthful = cards.iter().all(|card| card.rank == self.current_rank);

        for card in cards.iter() {
            self.pile.add(*card);
        }

        self.last_turn = Some((current_player_index, cards, is_truthful));
        self.current_player_index = Some((current_player_index + 1) % self.players.len());
        self.current_rank = self.current_rank.inc();
    }

    pub fn call_bs(&mut self, calling_player_index: usize) {
        let (last_player_index, last_cards, was_truthful) =
            self.last_turn.to_owned().expect("no last turn");

        if calling_player_index == last_player_index {
            panic!("cannot call BS on your own turn");
        }

        let affected_player_index = if was_truthful {
            calling_player_index
        } else {
            last_player_index
        };

        let calling_player = self
            .players
            .get_mut(affected_player_index)
            .expect("missing player");
        calling_player.add_cards(last_cards.as_slice());

        self.pile.clear();
        self.last_turn = None;
        self.current_player_index = Some(affected_player_index);
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        writeln!(f, "Current Rank: {}", self.current_rank)?;
        writeln!(f, "Pile: {}", self.pile)?;
        writeln!(f, "Last Turn: {:?}", self.last_turn)?;
        writeln!(f, "Current Player: {:?}", self.current_player_index)?;
        writeln!(f, "Players:")?;
        for player in self.players.iter() {
            writeln!(f, "{}", player)?;
        }
        Ok(())
    }
}
