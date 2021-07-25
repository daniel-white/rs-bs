use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, EnumIter, PartialOrd, Ord, PartialEq, Eq)]
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

#[derive(Copy, Clone, Debug, EnumIter, PartialOrd, Ord, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}
