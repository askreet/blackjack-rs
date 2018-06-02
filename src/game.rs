use deck::Deck;
use card::Card;
use std::result;
use std::fmt::Display;
use core::fmt;
use card::Rank;

#[derive(Debug, Eq, PartialEq)]
enum Error {
    NotEnoughChips,
}

type Result<T> = result::Result<T, Error>;

pub type Chips = u32;
pub type HandValue = u8;

pub struct Game {
    deck: Deck,
    pub dealer_cards: Vec<Card>,
    // TODO: accessors
    pub player_cards: Vec<Card>,
    pub player_money: Chips,
    pub current_bet: Chips,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: Deck::new(),
            dealer_cards: vec![],
            player_cards: vec![],
            player_money: 100,
            current_bet: 0,
        }
    }

    pub fn deal_hand(&mut self, bet: Chips) -> Result<()> {
        self.player_money -= bet;
        self.current_bet = bet;

        self.dealer_cards.push(self.deck.draw().unwrap());
        self.dealer_cards.push(self.deck.draw().unwrap());

        self.player_cards.push(self.deck.draw().unwrap());
        self.player_cards.push(self.deck.draw().unwrap());

        Ok(())
    }

    pub fn hit(&mut self) {
        self.player_cards.push(self.deck.draw().expect("Empty deck!"));
    }

    pub fn stand(&mut self) -> Vec<DealerAction> {
        let mut dealer_actions = vec![];

        while self.dealer_hand_value() <= 16 {
            let card = self.deck.draw().expect("Empty deck!");

            self.dealer_cards.push(card);
            dealer_actions.push(DealerAction::Hit(card));
        }

        if self.dealer_hand_value() > 21 {
            dealer_actions.push(DealerAction::Bust(self.dealer_hand_value()));
        } else {
            dealer_actions.push(DealerAction::Stand(self.dealer_hand_value()));
        }

        dealer_actions
    }

    fn dealer_hand_value(&self) -> HandValue {
        best_hand_value(self.dealer_ranks())
    }

    fn dealer_ranks(&self) -> Vec<Rank> {
        self.dealer_cards.iter().map(|c| c.rank).collect()
    }
}

pub enum Action {
    Hit,
    Stand,
}

#[derive(Debug, Eq, PartialEq)]
pub enum DealerAction {
    Hit(Card),
    Stand(HandValue),
    Bust(HandValue),
}

impl Display for DealerAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DealerAction::Hit(card) => write!(f, "Dealer hits: {}", card),
            DealerAction::Stand(value) => write!(f, "Dealer stands at {}", value),
            DealerAction::Bust(value) => write!(f, "Dealer busts with {}", value),
        }
    }
}

pub fn best_hand_value(ranks: Vec<Rank>) -> HandValue {
    let (aces, others): (Vec<Rank>, Vec<Rank>) =
        ranks.into_iter()
            .partition(|r| *r == Rank::Ace);

    let base: HandValue =
        others.iter()
            .fold(0, |a, e| a + e.value());

    aces.iter().fold(base, |a, _| {
        if a + 11 <= 21 {
            a + 11
        } else {
            a + 1
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use card::{Rank::*, Suit::*};

    #[test]
    fn dealing_a_hard_reduces_player_bank_and_deals_cards() {
        let mut game = Game {
            deck: Deck::new(),
            dealer_cards: Vec::new(),
            player_cards: Vec::new(),
            player_money: 100,
            current_bet: 0,
        };

        assert_eq!(Ok(()), game.deal_hand(5));

        assert_eq!(95, game.player_money);
        assert_eq!(5, game.current_bet);

        assert_eq!(2, game.player_cards.len());
        assert_eq!(2, game.dealer_cards.len());
    }

    #[test]
    fn hitting_adds_another_card_to_player_hand() {
        let mut game = Game {
            deck: Deck::new(),
            dealer_cards: cards!("2d", "3h"),
            player_cards: cards!("Tc", "As"),
            player_money: 0,
            current_bet: 0,
        };

        game.hit();

        assert_eq!(3, game.player_cards.len());
    }

    #[test]
    fn player_stands_and_dealer_wins() {
        let mut game = Game {
            deck: Deck::of(cards!("Th", "Tc")),
            dealer_cards: cards!("4h", "5h"),
            player_cards: cards!("Ts", "8s"),
            player_money: 0,
            current_bet: 0,
        };

        assert_eq!(
            vec![
                DealerAction::Hit(Card::of("Tc")),
                DealerAction::Stand(19),
            ],
            game.stand()
        );
    }

    #[test]
    fn player_stands_and_dealer_busts() {
        let mut game = Game {
            deck: Deck::of(cards!("Td", "Tc")),
            dealer_cards: cards!("6h", "Th"),
            player_cards: cards!("Ts", "8s"),
            player_money: 0,
            current_bet: 0,
        };

        assert_eq!(
            vec![
                DealerAction::Hit(Card::of("Tc")),
                DealerAction::Bust(26),
            ],
            game.stand()
        );
    }

    #[test]
    fn player_stands_and_dealer_hits_multiple_times() {
        let mut game = Game {
            deck: Deck::of(cards!["4s", "4h", "4c"]),
            dealer_cards: cards!["2s", "3h"],
            player_cards: cards!["Ah", "Kh"],
            player_money: 0,
            current_bet: 0,
        };

        assert_eq!(
            vec![
                DealerAction::Hit(Card::of("4c")),
                DealerAction::Hit(Card::of("4h")),
                DealerAction::Hit(Card::of("4s")),
                DealerAction::Stand(17),
            ],
            game.stand()
        );
    }

    #[test]
    fn ten_and_four_is_14() {
        assert_eq!(14, best_hand_value(vec![Ten, Four]))
    }

    #[test]
    fn jack_and_queen_is_20() {
        assert_eq!(20, best_hand_value(vec![Jack, Queen]))
    }

    #[test]
    fn simple_blackjack() {
        assert_eq!(21, best_hand_value(vec![King, Ace]))
    }

    #[test]
    fn two_aces() {
        assert_eq!(12, best_hand_value(vec![Ace, Ace]))
    }

    #[test]
    fn bust_with_two_aces() {
        assert_eq!(22, best_hand_value(vec![King, Ace, Ace]))
    }
}
