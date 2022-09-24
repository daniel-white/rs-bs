use linked_hash_set::{Iter, LinkedHashSet};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FmtResult};
use strum::IntoEnumIterator;

#[repr(u8)]
#[derive(Copy, Clone, Debug, EnumCount, EnumIter, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn inc(self) -> Self {
        match self {
            Self::King => Self::Ace,
            _ => Rank::iter()
                .nth(self as usize + 1)
                .expect("invalid rank index"),
        }
    }

    pub fn dec(self) -> Self {
        match self {
            Self::Ace => Self::King,
            _ => Rank::iter()
                .nth(self as usize - 1)
                .expect("invalid rank index"),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let rank_str = match self {
            Rank::Ace => "A".to_owned(),
            Rank::Jack => "J".to_owned(),
            Rank::Queen => "Q".to_owned(),
            Rank::King => "K".to_owned(),
            rank => (*rank as u8).to_string(),
        };
        write!(f, "{}", rank_str)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let suit_emoji = match self {
            Suit::Spades => "♠️",
            Suit::Hearts => "♥️",
            Suit::Clubs => "♣️",
            Suit::Diamonds => "♦️️",
        };
        write!(f, "{}", suit_emoji)
    }
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

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "[{}{} ]", self.rank, self.suit)
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct CardSet(LinkedHashSet<Card>);

impl Display for CardSet {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for card in self.0.iter() {
            write!(f, "{} ", card)?
        }
        Ok(())
    }
}

impl CardSet {
    pub fn new_empty() -> Self {
        Self(LinkedHashSet::with_capacity(52))
    }

    pub fn new_standard_deck() -> CardSet {
        let mut instance = Self::new_empty();

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

    pub fn add_all(&mut self, cards: &[Card]) {
        for card in cards {
            self.add(*card);
        }
    }

    fn reset(&mut self, cards: &[Card]) {
        self.clear();
        self.add_all(cards);
    }

    pub fn take_top(&mut self) -> Option<Card> {
        self.0.pop_front()
    }

    pub fn remove(&mut self, card: &Card) -> Option<Card> {
        if self.0.remove(card) {
            Some(*card)
        } else {
            None
        }
    }

    pub fn remove_all(&mut self, cards: &[Card]) {
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
        let clone = self.0.clone();
        let mut cards: Vec<Card> = clone.into_iter().collect();

        let mut rng = thread_rng();
        for i in 2..cards.len() {
            cards.swap(i, rng.gen_range(0..i + 1));
        }

        self.reset(&cards);
    }

    pub fn sort(&mut self) {
        let clone = self.0.clone();
        let mut cards: Vec<Card> = clone.into_iter().collect();
        cards.sort_by_key(|c| c.rank);
        self.reset(&cards);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
