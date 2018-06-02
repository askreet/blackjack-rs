#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Suit { Hearts, Diamonds, Spades, Clubs }

use self::Suit::*;

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Hearts, Diamonds, Spades, Clubs]
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Spades => "♤",
            Hearts => "♡",
            Diamonds => "♢",
            Clubs => "♧",
        })
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Rank { Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace }

use self::Rank::*;
use std::fmt::Display;

impl Rank {
    pub fn all() -> [Rank; 13] {
        [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace]
    }

    pub fn value(&self) -> u8 {
        match *self {
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten | Jack | Queen | King => 10,
            Ace => 11,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Two => "2", Three => "3", Four => "4", Five => "5",
            Six => "6", Seven => "7", Eight => "8", Nine => "9",
            Ten => "10", Jack => "J", Queen => "Q", King => "K",
            Ace => "A",
        })
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    #[cfg(test)]
    pub fn of(spec: &str) -> Card {
        let bytes = spec.as_bytes();

        let rank = match bytes[0] {
            b'2' => Two, b'3' => Three, b'4' => Four, b'5' => Five, b'6' => Six,
            b'7' => Seven, b'8' => Eight, b'9' => Nine, b'T' => Ten, b'J' => Jack,
            b'Q' => Queen, b'K' => King, b'A' => Ace,
            _ => panic!("Unknown card rank"),
        };

        let suit = match bytes[1] {
            b'h' => Hearts, b'd' => Diamonds, b'c' => Clubs, b's' => Spades,
            _ => panic!("Unknown card suit"),
        };

        Card { rank, suit }
    }
}

use std::fmt;
impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[cfg(test)]
macro_rules! cards {
    ( $( $spec:expr ),+ ) => {
        vec![$( Card::of($spec), )*]
    };
}

