mod deck;
mod holdem_hands;

use deck::Card;

fn main() {
    let mut deck = deck::Deck::new();
    deck.show();
    println!("---------");
    deck.reset();
    deck.show();
    println!("---------");
    let mut hand1: Vec<Card> = vec![];
    for _ in 0..7 {
        hand1.push(deck.pop_card());
    }
    println!("{:?}", hand1);
    let mut hand2: Vec<Card> = vec![];
    for _ in 0..7 {
        hand2.push(deck.pop_card());
    }
    println!("{:?}", hand2);
}
