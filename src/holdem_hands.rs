use crate::deck::Card;
use crate::deck::CardValue;
use std::collections::HashMap;

pub struct OneKicker {
    first: CardValue,
}

pub struct TwoKicker {
    first: CardValue,
    second: CardValue,
}
pub struct ThreeKicker {
    first: CardValue,
    second: CardValue,
    third: CardValue,
}

pub struct FiveKicker {
    first: CardValue,
    second: CardValue,
    third: CardValue,
    fourth: CardValue,
    fifth: CardValue,
}

pub enum Hand {
    HighCard {
        kickers: FiveKicker,
    },
    Pair {
        value: CardValue,
        kickers: ThreeKicker,
    },
    TwoPair {
        first_value: CardValue,
        second_value: CardValue,
        kickers: OneKicker,
    },
    Set {
        value: CardValue,
        kickers: TwoKicker,
    },
    Straight {
        high_card: CardValue,
    },
    Flush {
        high_card: CardValue,
    },
    FullHouse {
        two_value: CardValue,
        three_value: CardValue,
    },
    FourOfAKind {
        value: CardValue,
        kicker: OneKicker,
    },
    StraightFlush {
        high_card: CardValue,
    },
    RoyaleFlush,
}

/// Given a vector of vectors of cards (representing the hands of the players),
/// returns a vector of rankings. The ranking in a particluar index in the returned
/// vector corresponds to the ranking of the hand at that index in the input. If two
/// hands have the same strength, they will have the same ranking in the returned vector.
/// Each of the input vectors must be seven cards, representing the 5 community cards and
/// the 2 unique cards for that player.
pub fn assign_hand_rankings(hands: Vec<Vec<Card>>) -> Vec<u8> {
    // get the best possible hand for each player
    let mut best_hands: Vec<Hand> = vec![];
    for hand in hands {
        best_hands.push(get_best_hand(hand));
    }

    // assign rankings
    rank_hands(best_hands)
}

/// Given a vector of 7 cards, representing the 5 community cards and the 2
/// unique cards for a player, returns the best possible hand that can be constructed
pub fn get_best_hand(hand: Vec<Card>) -> Hand {
    let mut hand_names: Vec<Hand> = Vec::new();
    let combos = generate_combinations(hand, vec![], 0);
    for combo in combos {
        let royal_flush = get_royale_flush(combo);
        if let Some(val) = royal_flush {
            hand_names.push(val);
            continue;
        }
        // let straight_flush = is_straight_flush(combo);
        // if let Some(val) = straight_flush {
        //     hand_names.push(val);
        //     continue;
        // }
        // let four_of_a_kind = is_four_of_a_kind(combo);
        // if let Some(val) = four_of_a_kind {
        //     hand_names.push(val);
        //     continue;
        // }
        // let full_house = is_full_house(combo);
        // if let Some(val) = full_house {
        //     hand_names.push(val);
        //     continue;
        // }
        // let flush = is_flush(combo);
        // if let Some(val) = flush {
        //     hand_names.push(val);
        //     continue;
        // }
        // let set = is_set(combo);
        // if let Some(val) = set {
        //     hand_names.push(val);
        //     continue;
        // }
        // let two_pair = is_two_pair(combo);
        // if let Some(val) = two_pair {
        //     hand_names.push(val);
        //     continue;
        // }
        // let pair = is_pair(combo);
        // if let Some(val) = pair {
        //     hand_names.push(val);
        //     continue;
        // }
        // hand_names.push(get_high_card(combo));
    }
    let rankings = rank_hands(hand_names);
    for (index, ranking) in rankings.iter().enumerate() {
        if *ranking == 1 {
            return hand_names[index];
        }
    }

    panic!("No best hand was found");
}

/// Given a vector of hands, returns the rankings of each hand, where the value of some index
/// in the returned vector corresponds to the ranking of that hand in the input vector. If two
/// hands have the same strength according to the rules of texas holdem then they will have the
/// same ranking in the returned vector.
fn rank_hands(hands: Vec<Hand>) -> Vec<u8> {
    // let mut current_ranking: u8 = 1;
    // let mut ranings: Vec<u8> = vec![];
    // for hand in hand {
    //     match hand {}
    // }
}

/// Returns the 21 combinations of 5 cards given 7 cards.
fn generate_combinations(hand: Vec<Card>, cur_combo: Vec<Card>, index: usize) -> Vec<Vec<Card>> {
    if index == hand.len() {
        if cur_combo.len() == 5 {
            return vec![cur_combo];
        } else {
            return vec![];
        }
    }
    if cur_combo.len() == 5 {
        return vec![cur_combo];
    }

    // add the card
    let added = generate_combinations(
        hand.clone(),
        [cur_combo.clone(), vec![hand[index]]].concat(),
        index + 1,
    );
    // don't add the card
    let skipped = generate_combinations(hand.clone(), cur_combo.clone(), index + 1);

    [added, skipped].concat()
}

fn is_flush(hand: Vec<Card>) -> bool {
    let suit = hand[0].suit;
    for i in 0..hand.len() {
        if suit != hand[i].suit {
            return false;
        }
    }

    return true;
}

fn get_royale_flush(hand: Vec<Card>) -> Option<Hand> {
    if !is_flush(hand) {
        return None;
    }
    let mut sum = 0;
    for card in hand {
        sum += card.value.value();
    }
    if sum == 50 {
        return Some(Hand::RoyaleFlush);
    }

    None
}

fn get_straight_flush(hand: Vec<Card>) -> Option<Hand> {
    if !is_flush(hand) {
        return None;
    }

    let mut values = vec![];
    for card in hand {
        values.push(card.value.value())
    }
    values.sort();

    for index in 1..values.len() {
        if values[index] - 1 != values[index - 1] {
            return None;
        }
    }

    Some(Hand::StraightFlush {
        high_card: CardValue::new(values[values.len() - 1]),
    })
}

fn get_four_of_a_kind(hand: Vec<Card>) -> Option<Hand> {
    let mut value_map = HashMap::new();
    for card in hand {
        let mut count = value_map.entry(card.value.value()).or_insert(1);
        *count += 1
    }

    for (four_val, four_count) in &value_map {
        if *four_count == 4 {
            for (one_val, one_count) in &value_map {
                if *one_count == 1 {
                    return Some(Hand::FourOfAKind {
                        value: CardValue::new(*four_val),
                        kicker: {
                            OneKicker {
                                first: CardValue::new(*one_val),
                            }
                        },
                    });
                }
            }
        }
    }

    None
}

fn get_full_house(hand: Vec<Card>) -> Option<Hand> {
    let mut value_map = HashMap::new();
    for card in hand {
        let mut count = value_map.entry(card.value.value()).or_insert(1);
        *count += 1
    }

    for (three_val, three_count) in &value_map {
        if *three_count == 3 {
            for (two_val, two_count) in &value_map {
                if *two_count == 2 {
                    return Some(Hand::FullHouse {
                        two_value: CardValue::new(*two_val),
                        three_value: CardValue::new(*three_val),
                    });
                }
            }
        }
    }

    None
}

fn get_flush(hand: Vec<Card>) -> Option<Hand> {
    if !is_flush(hand) {
        return None;
    }

    let mut max = CardValue::Two;
    for card in hand {
        if card.value.value() > max.value() {
            max = card.value
        }
    }

    Some(Hand::Flush { high_card: max })
}

fn get_straight(hand: Vec<Card>) -> Option<Hand> {
    let mut values = vec![];
    for card in hand {
        values.push(card.value.value());
    }
    values.sort();
    for index in 1..values.len() {
        if values[index] - 1 != values[index - 1] {
            return None;
        }
    }

    Some(Hand::Straight {
        high_card: CardValue::new(values[values.len() - 1]),
    })
}

// fn get_set(hand: Vec<Card>) -> Option<Hand> {}
// fn get_two_pair(hand: Vec<Card>) -> Option<Hand> {}
// fn get_pair(hand: Vec<Card>) -> Option<Hand> {}

fn get_high_card(hand: Vec<Card>) -> Hand {
    let mut values = vec![];
    for card in hand {
        values.push(card.value.value());
    }
    values.sort();

    Hand::HighCard {
        kickers: FiveKicker {
            first: CardValue::new(values[5]),
            second: CardValue::new(values[4]),
            third: CardValue::new(values[3]),
            fourth: CardValue::new(values[2]),
            fifth: CardValue::new(values[1]),
        },
    }
}
