use specs::prelude::*;
use specs::Component;

pub mod cards;

use cards::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Deck;

pub fn create_world() -> World {
    let mut world = World::new();
    world.register::<Cards>();
    world.register::<Player>();
    world.register::<Deck>();

    let deck = world
        .create_entity()
        .with(Deck)
        .with(Cards::standard_deck())
        .build();
    let player = world
        .create_entity()
        .with(Player)
        .with(Cards::default())
        .build();
    let player2 = world
        .create_entity()
        .with(Player)
        .with(Cards::default())
        .build();
    let player3 = world
        .create_entity()
        .with(Player)
        .with(Cards::default())
        .build();

    let mut deal = Deal {
        deck,
        players: vec![player, player2, player3],
    };
    deal.run_now(&world);
    world.maintain();
    world
}

struct Deal {
    deck: Entity,
    players: Vec<Entity>,
}

impl<'a> System<'a> for Deal {
    type SystemData = WriteStorage<'a, Cards>;

    fn run(&mut self, mut cards: Self::SystemData) {
        let mut deck = cards.get(self.deck).unwrap().clone();
        deck.shuffle();

        for player in self.players.iter().cycle() {
            let player_cards = cards.get_mut(*player).unwrap();
            if let Some(card) = deck.take_top() {
                player_cards.add(card);
            } else {
                break;
            }
        }

        let mut deck = cards.get_mut(self.deck).unwrap();
        deck.clear();

        for player in self.players.iter() {
            println!("====PLAYER====");
            let player_cards = cards.get_mut(*player).unwrap();
            player_cards.sort_by_rank();
            for card in player_cards.iter() {
                println!("{:?}", card);
            }
        }
    }
}

// use deck::*;
// use hand::*;
//
// type TurnCards = [Option<Card>; 4];
//
// #[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
// pub struct Turn {
//     player: usize,
//     claimed_rank: Rank,
//     cards: TurnCards,
//     was_truthful: bool,
// }
//
// impl Turn {
//     pub fn attempt(player: usize, claimed_rank: Rank, cards: TurnCards) -> Result<Self, ()> {
//         match cards.iter().filter(|c| c.is_some()).count() {
//             0 => Err(()),
//             _ => Ok(Self {
//                 player,
//                 claimed_rank,
//                 cards,
//                 was_truthful: cards
//                     .iter()
//                     .filter(|c| c.is_some())
//                     .all(|c| c.unwrap().rank == claimed_rank),
//             }),
//         }
//     }
// }
//
// #[derive(Debug)]
// pub struct Player {
//     pub id: usize,
//     pub name: String,
//     pub hand: Hand,
// }
//
// #[derive(Debug)]
// pub struct Game {
//     pub players: Vec<Player>,
//     current_player: usize,
//     current_rank: Rank,
//     pub pile: Vec<Turn>,
// }
//
// impl Game {
//     pub fn new() -> Self {
//         Self {
//             players: Vec::default(),
//             current_player: 0,
//             current_rank: Rank::Ace,
//             pile: Vec::default(),
//         }
//     }
//
//     pub fn add_player(&mut self) {
//         self.players.push(Player {
//             id: self.players.len(),
//             name: "aaa".to_string(),
//             hand: Hand::default(),
//         })
//     }
//     pub fn start(&mut self) {
//         let mut deck = Deck::default();
//         deck.shuffle();
//         let mut count = 0;
//
//         while let Some(card) = deck.draw() {
//             let hand_index = count % self.players.len();
//             self.players[hand_index].hand.add(card);
//             count += 1;
//         }
//     }
//
//     pub fn try_turn(&mut self, player: usize, cards: TurnCards) -> Result<(), ()> {
//         if player != self.current_player {
//             return Err(());
//         }
//
//         let current_player = self.players.get(self.current_player).ok_or(())?;
//         let has_cards = cards
//             .iter()
//             .filter(|c| c.is_some())
//             .map(|c| c.unwrap())
//             .all(|c| current_player.hand.contains(&c));
//
//         if !has_cards {
//             return Err(());
//         }
//
//         let turn = Turn::attempt(player, self.current_rank, cards)?;
//         self.pile.push(turn);
//
//         Ok(())
//     }
//
//     pub fn challenge(&mut self, player: usize) -> Result<(), ()> {
//         let turn = self.pile.last().ok_or(())?;
//
//         let unlucky_player_id = if turn.was_truthful {
//             player
//         } else {
//             turn.player
//         };
//
//         let unlucky_player = self.players.get_mut(unlucky_player_id).ok_or(())?;
//         let pile_cards = self
//             .pile
//             .iter()
//             .flat_map(|t| t.cards)
//             .filter(|c| c.is_some());
//
//         for card in pile_cards {
//             unlucky_player.hand.add(card.unwrap());
//         }
//
//         self.pile.clear();
//
//         Ok(())
//     }
// }
