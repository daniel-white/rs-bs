use derive_more::{Display, Error};
use linked_hash_set::{Iter, LinkedHashSet};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use strum::IntoEnumIterator;

#[derive(Copy, Clone, Debug, EnumCount, EnumIter, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn inc(self) -> Self {
        if self == Self::King {
            Self::Ace
        } else {
            Rank::iter()
                .nth(self as usize + 1)
                .expect("invalid rank index")
        }
    }

    pub fn dec(self) -> Self {
        if self == Self::Ace {
            Self::King
        } else {
            Rank::iter()
                .nth(self as usize - 1)
                .expect("invalid rank index")
        }
    }
}

#[test]
pub fn rank_inc() {
    assert_eq!(Rank::Two.inc(), Rank::Three);
}

#[test]
pub fn rank_inc_wrap() {
    assert_eq!(Rank::King.inc(), Rank::Ace);
}

#[test]
pub fn rank_dec() {
    assert_eq!(Rank::Two.dec(), Rank::Ace);
}

#[test]
pub fn rank_dec_wrap() {
    assert_eq!(Rank::Ace.dec(), Rank::King);
}

#[derive(Copy, Clone, Debug, EnumIter, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.suit == other.suit {
            self.rank.cmp(&other.rank)
        } else {
            self.suit.cmp(&other.suit)
        }
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct CardSet(LinkedHashSet<Card>);

impl CardSet {
    pub fn new() -> Self {
        Self(LinkedHashSet::new())
    }

    pub fn standard_deck() -> Self {
        let mut instance = Self(LinkedHashSet::with_capacity(52));

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                instance.add(Card::new(rank, suit));
            }
        }

        instance
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.0.contains(card)
    }

    pub fn add(&mut self, card: Card) {
        self.0.insert_if_absent(card);
    }

    pub fn add_all(&mut self, cards: &[&Card]) {
        for card in cards {
            self.add(**card);
        }
    }

    pub fn take_top(&mut self) -> Option<Card> {
        self.0.pop_front()
    }

    pub fn remove(&mut self, card: &Card) {
        self.0.remove(card);
    }

    pub fn remove_all(&mut self, cards: &[&Card]) {
        for card in cards {
            self.remove(card);
        }
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter(&self) -> Iter<'_, Card> {
        self.0.iter()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let clone = self.0.clone();
        let mut cards: Vec<&Card> = clone.iter().collect();
        self.clear();

        let mut i = cards.len();
        while i >= 2 {
            i -= 1;
            cards.swap(i, rng.gen_range(0..i + 1));
        }

        for card in cards {
            self.0.insert(*card);
        }
    }

    pub fn sort(&mut self) {
        let clone = self.0.clone();
        let mut cards: Vec<&Card> = clone.iter().collect();
        self.clear();

        cards.sort();

        for card in cards {
            self.0.insert(*card);
        }
    }

    pub fn sort_by_rank(&mut self) {
        let clone = self.0.clone();
        let mut cards: Vec<&Card> = clone.iter().collect();
        self.clear();

        cards.sort_by_key(|c| c.rank);

        for card in cards {
            self.0.insert(*card);
        }
    }
}
