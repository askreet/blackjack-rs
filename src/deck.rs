extern crate rand;

use card::{Card, Suit, Rank};
use self::rand::{Rng, thread_rng};

#[derive(Eq, PartialEq, Debug)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = vec![];

        for suit in Suit::all().iter() {
            for rank in Rank::all().iter() {
                cards.push(Card::new(*rank, *suit));
            }
        }

        thread_rng().shuffle(&mut cards);

        Deck { cards }
    }

    pub fn of(cards: Vec<Card>) -> Deck {
        Deck { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[cfg(test)]
mod tests {
    use deck::Deck;
    use card::{Card, Rank, Rank::*, Suit, Suit::*};

    #[test]
    fn a_new_deck_has_52_cards() {
        let d = Deck::new();

        assert_eq!(52, d.cards.len())
    }

    #[test]
    fn a_new_deck_has_13_of_each_suit() {
        let d = Deck::new();
        let cards = d.cards;

        for suit in Suit::all().iter() {
            assert_eq!(13, cards.iter().filter(|c| c.suit == *suit).count())
        }
    }

    #[test]
    fn a_new_deck_has_4_of_each_rank() {
        let d = Deck::new();
        let cards = d.cards;

        for rank in Rank::all().iter() {
            assert_eq!(4, cards.iter().filter(|c| c.rank == *rank).count());
        }
    }

    #[test]
    fn new_decks_should_be_shuffled() {
        let d1 = Deck::new();
        let d2 = Deck::new();

        assert_ne!(d1, d2);
    }
}
