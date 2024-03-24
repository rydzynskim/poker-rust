mod deck;
mod hand;

fn main() {
    // initialize a new deck and shuffle it
    let mut deck = deck::Deck::new();
    deck.shuffle();

    // deal cards to 9 players
    let common = deck.pop_cards(5).unwrap();
    let mut players: Vec<deck::CardCollection> = vec![];
    for _i in 0..9 {
        players.push(deck::CardCollection::concat(
            common.clone(),
            deck.pop_cards(2).unwrap(),
        ));
    }

    // get the best hand for every player
    let mut best_hands = vec![];
    for player in &players {
        best_hands.push(hand::get_best_hand(player.clone()));
    }

    // rank the hands in relation to each other
    let rankings = hand::assign_hand_rankings(players.clone());
    let mut rankings_with_indices: Vec<_> = rankings.iter().enumerate().collect();
    rankings_with_indices.sort_by(|a, b| a.1.cmp(&b.1));
    for (index, rank) in rankings_with_indices {
        println!("-----------------");
        println!("{}", players[index]);
        println!("{}", best_hands[index]);
        println!("{}", rank);
        println!("-----------------");
    }
}
