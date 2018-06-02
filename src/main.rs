extern crate core;

use game::{Action, Action::*, Game};
use card::Card;
use deck::Deck;
use std::io;
use std::io::Write;

#[macro_use]
mod card;
mod deck;
mod game;

fn main() {
    let mut game = Game::new();

    loop {
        let action = pick_action(&game);
        match action {
            Action::Hit => { game.hit(); },
            Action::Stand => { game.stand(); },
        }
//        game.resolve_action(action);
    }
}

fn pick_action(game: &Game) -> Action {
    println!("Bank: {}", game.player_money);
    println!("Current Bet: ?");
    print!("Dealer Cards: ");

    for dealer_card in game.dealer_cards.iter() {
        print!("{}", dealer_card);
    }

    print!("\nPlayer Cards: ");

    for player_card in game.player_cards.iter() {
        print!("{}", player_card);
    }

    println!("\nActions:");
    println!(" H) Hit");
    println!(" S) Stand");

    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "H" => return Hit,
            "S" => return Stand,
            _ => {}
        }
    }
}

