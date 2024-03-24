use crate::deck::CardCollection;
use crate::deck::CardValue;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct StraightFlushHand {
    high_card: CardValue,
}

#[derive(Clone)]
pub struct FourOfAKindHand {
    value: CardValue,
}

#[derive(Clone)]
pub struct FullHouseHand {
    three_value: CardValue,
}

#[derive(Clone)]
pub struct FlushHand {
    high_card: CardValue,
}

#[derive(Clone)]
pub struct StraightHand {
    high_card: CardValue,
}

#[derive(Clone)]
pub struct SetHand {
    value: CardValue,
}

#[derive(Clone)]
pub struct TwoPairHand {
    first_value: CardValue,
    second_value: CardValue,
    kicker: CardValue,
}

#[derive(Clone)]
pub struct PairHand {
    value: CardValue,
    first_kicker: CardValue,
    second_kicker: CardValue,
    third_kicker: CardValue,
}

#[derive(Clone)]
pub struct HighCardHand {
    first_kicker: CardValue,
    second_kicker: CardValue,
    third_kicker: CardValue,
    fourth_kicker: CardValue,
    fifth_kicker: CardValue,
}

#[derive(Clone)]
pub enum Hand {
    HighCard(HighCardHand),
    Pair(PairHand),
    TwoPair(TwoPairHand),
    Set(SetHand),
    Straight(StraightHand),
    Flush(FlushHand),
    FullHouse(FullHouseHand),
    FourOfAKind(FourOfAKindHand),
    StraightFlush(StraightFlushHand),
    RoyaleFlush,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Hand::RoyaleFlush => {
                write!(f, "Royale Flush")
            }
            Hand::StraightFlush { .. } => {
                write!(f, "Straight Flush")
            }
            Hand::FourOfAKind { .. } => {
                write!(f, "Four of a Kind")
            }
            Hand::FullHouse { .. } => {
                write!(f, "Full House")
            }
            Hand::Flush { .. } => {
                write!(f, "Flush")
            }
            Hand::Straight { .. } => {
                write!(f, "Straight")
            }
            Hand::Set { .. } => {
                write!(f, "Set")
            }
            Hand::TwoPair { .. } => {
                write!(f, "Two Pair")
            }
            Hand::Pair { .. } => {
                write!(f, "Pair")
            }
            Hand::HighCard { .. } => {
                write!(f, "High Card")
            }
        }
    }
}

// -----------------------
// STABILITY NOTE
// The functions below can definitely be improved. These functions were written quickly - the number
// of iterations should be cut down, the logic cleaned up, and more comments added.
// Additionally, an interface that exposes a more iterative style of computation,
// where we track the best hand so far and see how the addition of another card changes that, could be considered.
// Also, might make more sense for these to exist as static methods on the Hand struct.
// -----------------------

/// Given a vector of vectors of cards (representing the hands of the players),
/// returns a vector of rankings. The ranking in a particluar index in the returned
/// vector corresponds to the ranking of the hand at that index in the input. If two
/// hands have the same strength, they will have the same ranking in the returned vector.
/// Each of the input vectors must be seven cards, representing the 5 community cards and
/// the 2 unique cards for that player.
pub fn assign_hand_rankings(hands: Vec<CardCollection>) -> Vec<u8> {
    // get the best possible hand for each player
    let mut best_hands: Vec<Hand> = vec![];
    for hand in hands {
        best_hands.push(get_best_hand(hand));
    }

    // assign rankings
    rank_hands(best_hands)
}

/// Given a reference to a card collection of 7 cards, representing the 5 community cards and the 2
/// unique cards for a player, returns the best possible hand that can be constructed.
pub fn get_best_hand(cards: CardCollection) -> Hand {
    let mut hand_names: Vec<Hand> = Vec::new();
    // get the best hand for each combo
    for combo in generate_combinations(cards, CardCollection(vec![]), 0) {
        let royal_flush = get_royale_flush(combo.clone());
        if let Some(val) = royal_flush {
            hand_names.push(val);
            continue;
        }
        let straight_flush = get_straight_flush(combo.clone());
        if let Some(val) = straight_flush {
            hand_names.push(val);
            continue;
        }
        let four_of_a_kind = get_four_of_a_kind(combo.clone());
        if let Some(val) = four_of_a_kind {
            hand_names.push(val);
            continue;
        }
        let full_house = get_full_house(combo.clone());
        if let Some(val) = full_house {
            hand_names.push(val);
            continue;
        }
        let flush = get_flush(combo.clone());
        if let Some(val) = flush {
            hand_names.push(val);
            continue;
        }
        let straight = get_straight(combo.clone());
        if let Some(val) = straight {
            hand_names.push(val);
            continue;
        }
        let set = get_set(combo.clone());
        if let Some(val) = set {
            hand_names.push(val);
            continue;
        }
        let two_pair = get_two_pair(combo.clone());
        if let Some(val) = two_pair {
            hand_names.push(val);
            continue;
        }
        let pair = get_pair(combo.clone());
        if let Some(val) = pair {
            hand_names.push(val);
            continue;
        }
        hand_names.push(get_high_card(combo.clone()));
    }

    // rank all the hands and return the one with the highest ranking
    let rankings = rank_hands(hand_names.clone());
    for (index, ranking) in rankings.iter().enumerate() {
        if *ranking == 1 {
            return hand_names[index].clone();
        }
    }

    panic!("No best hand was found {:?}", rankings);
}

/// Given a vector of hands, returns the rankings of each hand, where the value of some index
/// in the returned vector corresponds to the ranking of that hand in the input vector. If two
/// hands have the same strength according to the rules of texas holdem then they will have the
/// same ranking in the returned vector.
fn rank_hands(hands: Vec<Hand>) -> Vec<u8> {
    let order = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut rankings: Vec<u8> = vec![0; hands.len()];
    let mut rank_counter: u8 = 1;
    for hand_type in order {
        let mut considered_hands = vec![];
        let mut considered_indices = vec![];
        for (index, hand) in hands.iter().enumerate() {
            match hand {
                Hand::RoyaleFlush => {
                    if hand_type == 1 {
                        considered_hands.push(hand.clone());
                        considered_indices.push(index);
                    }
                }
                Hand::StraightFlush { .. } => {
                    if hand_type == 2 {
                        considered_hands.push(hand.clone());
                        considered_indices.push(index);
                    }
                }
                Hand::FourOfAKind { .. } => {
                    if hand_type == 3 {
                        considered_hands.push(hand.clone());
                        considered_indices.push(index);
                    }
                }
                Hand::FullHouse { .. } => {
                    if hand_type == 4 {
                        considered_hands.push(hand.clone());
                        considered_indices.push(index);
                    }
                }
                Hand::Flush { .. } => {
                    if hand_type == 5 {
                        considered_hands.push(hand.clone());
                        considered_indices.push(index);
                    }
                }
                Hand::Straight { .. } => {
                    if hand_type == 6 {
                        considered_hands.push(hand.clone());
                        considered_indices.push(index);
                    }
                }
                Hand::Set { .. } => {
                    if hand_type == 7 {
                        considered_hands.push(hand.clone());
                        considered_indices.push(index);
                    }
                }
                Hand::TwoPair { .. } => {
                    if hand_type == 8 {
                        considered_hands.push(hand.clone());
                        considered_indices.push(index);
                    }
                }
                Hand::Pair { .. } => {
                    if hand_type == 9 {
                        considered_hands.push(hand.clone());
                        considered_indices.push(index);
                    }
                }
                Hand::HighCard { .. } => {
                    if hand_type == 10 {
                        considered_hands.push(hand.clone());
                        considered_indices.push(index);
                    }
                }
            };
        }
        if considered_hands.len() == 0 {
            continue;
        } else if considered_hands.len() == 1 {
            rankings[considered_indices[0]] = rank_counter;
            rank_counter += 1;
        } else {
            for (index, rank) in tie_breaker(considered_hands.clone()).iter().enumerate() {
                rankings[considered_indices[index]] = *rank + rank_counter - 1;
            }
            let max = rankings.iter().max();
            match max {
                Some(val) => {
                    rank_counter = val + 1;
                }
                None => {
                    panic!("saw a rankings vector without any elements");
                }
            }
        }
    }

    rankings
}

/// Given a vector of hands of the same variant, returns their rankings considering
/// values and kickers.
fn tie_breaker(hands: Vec<Hand>) -> Vec<u8> {
    match hands[0] {
        Hand::RoyaleFlush => {
            // all royale flushes are of equal strength
            vec![1u8; hands.len()]
        }
        Hand::StraightFlush { .. } => {
            // Convert Vec<Hand> to Vec<StraightFlushHand>
            let straight_flush_hands: Vec<StraightFlushHand> = hands
                .into_iter()
                .filter_map(|hand| {
                    if let Hand::StraightFlush(straight_flush_hand) = hand {
                        Some(straight_flush_hand)
                    } else {
                        panic!("expected only straight flush hands, but saw something else");
                    }
                })
                .collect();

            straight_flush_tie_breaker(straight_flush_hands)
        }
        Hand::FourOfAKind { .. } => {
            // Convert Vec<Hand> to Vec<FourOfAKindHand>
            let four_of_a_kind_hands: Vec<FourOfAKindHand> = hands
                .into_iter()
                .filter_map(|hand| {
                    if let Hand::FourOfAKind(four_of_a_kind_hand) = hand {
                        Some(four_of_a_kind_hand)
                    } else {
                        panic!("expected only four of a kind hands, but saw something else");
                    }
                })
                .collect();

            four_of_a_kind_tie_breaker(four_of_a_kind_hands)
        }
        Hand::FullHouse { .. } => {
            // Convert Vec<Hand> to Vec<FullHouseHand>
            let full_house_hands: Vec<FullHouseHand> = hands
                .into_iter()
                .filter_map(|hand| {
                    if let Hand::FullHouse(full_house_hand) = hand {
                        Some(full_house_hand)
                    } else {
                        panic!("expected only full house hands, but saw something else");
                    }
                })
                .collect();

            full_house_tie_breaker(full_house_hands)
        }
        Hand::Flush { .. } => {
            // Convert Vec<Hand> to Vec<FlushHand>
            let flush_hands: Vec<FlushHand> = hands
                .into_iter()
                .filter_map(|hand| {
                    if let Hand::Flush(flush_hand) = hand {
                        Some(flush_hand)
                    } else {
                        panic!("expected only flush hands, but saw something else");
                    }
                })
                .collect();

            flush_tie_breaker(flush_hands)
        }
        Hand::Straight { .. } => {
            // Convert Vec<Hand> to Vec<StraightHand>
            let straight_hands: Vec<StraightHand> = hands
                .into_iter()
                .filter_map(|hand| {
                    if let Hand::Straight(straight_hand) = hand {
                        Some(straight_hand)
                    } else {
                        panic!("expected only straight hands, but saw something else");
                    }
                })
                .collect();

            straight_tie_breaker(straight_hands)
        }
        Hand::Set { .. } => {
            // Convert Vec<Hand> to Vec<SetHand>
            let set_hands: Vec<SetHand> = hands
                .into_iter()
                .filter_map(|hand| {
                    if let Hand::Set(set_hand) = hand {
                        Some(set_hand)
                    } else {
                        panic!("expected only set hands, but saw something else");
                    }
                })
                .collect();

            set_tie_breaker(set_hands)
        }
        Hand::TwoPair { .. } => {
            // Convert Vec<Hand> to Vec<TwoPairHand>
            let two_pair_hands: Vec<TwoPairHand> = hands
                .into_iter()
                .filter_map(|hand| {
                    if let Hand::TwoPair(two_pair_hand) = hand {
                        Some(two_pair_hand)
                    } else {
                        panic!("expected only two pair hands, but saw something else");
                    }
                })
                .collect();

            two_pair_tie_breaker(two_pair_hands)
        }
        Hand::Pair { .. } => {
            // Convert Vec<Hand> to Vec<PairHand>
            let pair_hands: Vec<PairHand> = hands
                .into_iter()
                .filter_map(|hand| {
                    if let Hand::Pair(pair_hand) = hand {
                        Some(pair_hand)
                    } else {
                        panic!("expected only pair hands, but saw something else");
                    }
                })
                .collect();

            pair_tie_breaker(pair_hands)
        }
        Hand::HighCard { .. } => {
            // Convert Vec<Hand> to Vec<HighCardHand>
            let high_card_hands: Vec<HighCardHand> = hands
                .into_iter()
                .filter_map(|hand| {
                    if let Hand::HighCard(high_card_hand) = hand {
                        Some(high_card_hand)
                    } else {
                        panic!("expected only high card hands, but saw something else");
                    }
                })
                .collect();

            high_card_tie_breaker(high_card_hands)
        }
    }
}

/// Returns the 21 combinations of 7 choose 5 cards.
fn generate_combinations(
    cards: CardCollection,
    cur_combo: CardCollection,
    index: usize,
) -> Vec<CardCollection> {
    if index == cards.0.len() {
        if cur_combo.0.len() == 5 {
            return vec![cur_combo];
        } else {
            return vec![];
        }
    }
    if cur_combo.0.len() == 5 {
        return vec![cur_combo];
    }

    let added = generate_combinations(
        cards.clone(),
        CardCollection::concat(
            cur_combo.clone(),
            CardCollection(vec![cards.0[index].clone()]),
        ),
        index + 1,
    );
    let skipped = generate_combinations(cards.clone(), cur_combo.clone(), index + 1);

    [added, skipped].concat()
}

fn is_flush(hand: CardCollection) -> bool {
    for i in 0..hand.0.len() {
        if hand.0[0].suit != hand.0[i].suit {
            return false;
        }
    }

    return true;
}

fn get_royale_flush(hand: CardCollection) -> Option<Hand> {
    if !is_flush(hand.clone()) {
        return None;
    }
    let mut sum = 0;
    for card in hand.0 {
        sum += card.value.value();
    }
    if sum == 50 {
        return Some(Hand::RoyaleFlush);
    }

    None
}

fn get_straight_flush(hand: CardCollection) -> Option<Hand> {
    if !is_flush(hand.clone()) {
        return None;
    }

    let mut values = vec![];
    for card in hand.0 {
        values.push(card.value.value())
    }
    values.sort();

    for index in 1..values.len() {
        if values[index] - 1 != values[index - 1] {
            return None;
        }
    }

    Some(Hand::StraightFlush(StraightFlushHand {
        high_card: CardValue::new(values[values.len() - 1]),
    }))
}

fn get_four_of_a_kind(hand: CardCollection) -> Option<Hand> {
    let mut value_map = HashMap::new();
    for card in hand.0 {
        let count = value_map.entry(card.value.value()).or_insert(0);
        *count += 1
    }

    for (four_val, four_count) in &value_map {
        if *four_count == 4 {
            for (_one_val, one_count) in &value_map {
                if *one_count == 1 {
                    return Some(Hand::FourOfAKind(FourOfAKindHand {
                        value: CardValue::new(*four_val),
                    }));
                }
            }
        }
    }

    None
}

fn get_full_house(hand: CardCollection) -> Option<Hand> {
    let mut value_map = HashMap::new();
    for card in hand.0 {
        let count = value_map.entry(card.value.value()).or_insert(0);
        *count += 1
    }

    for (three_val, three_count) in &value_map {
        if *three_count == 3 {
            for (_two_val, two_count) in &value_map {
                if *two_count == 2 {
                    return Some(Hand::FullHouse(FullHouseHand {
                        three_value: CardValue::new(*three_val),
                    }));
                }
            }
        }
    }

    None
}

fn get_flush(hand: CardCollection) -> Option<Hand> {
    if !is_flush(hand.clone()) {
        return None;
    }

    let mut max = CardValue::Two;
    for card in hand.0 {
        if card.value.value() > max.value() {
            max = card.value
        }
    }

    Some(Hand::Flush(FlushHand { high_card: max }))
}

fn get_straight(hand: CardCollection) -> Option<Hand> {
    let mut values = vec![];
    for card in hand.0 {
        values.push(card.value.value());
    }
    values.sort();
    for index in 1..values.len() {
        if values[index] - 1 != values[index - 1] {
            return None;
        }
    }

    Some(Hand::Straight(StraightHand {
        high_card: CardValue::new(values[values.len() - 1]),
    }))
}

fn get_set(hand: CardCollection) -> Option<Hand> {
    let mut value_map = HashMap::new();
    for card in hand.0.clone() {
        let count = value_map.entry(card.value.value()).or_insert(0);
        *count += 1
    }

    let mut other_values = vec![];
    for (val, count) in &value_map {
        if *count == 3 {
            for card in hand.0.clone() {
                if card.value.value() != *val {
                    other_values.push(card.value.value());
                }
                other_values.sort();
                return Some(Hand::Set(SetHand {
                    value: CardValue::new(*val),
                }));
            }
        }
    }

    None
}

fn get_two_pair(hand: CardCollection) -> Option<Hand> {
    let mut value_map = HashMap::new();
    for card in hand.0 {
        let count = value_map.entry(card.value.value()).or_insert(0);
        *count += 1
    }

    let mut one_pair = false;
    let mut two_pair = false;
    let mut pair_values = vec![];
    for (val, count) in &value_map {
        if *count == 2 {
            if one_pair {
                two_pair = true;
            } else {
                one_pair = true;
            }
            pair_values.push(*val)
        }
    }

    if two_pair {
        pair_values.sort();
        for (val, count) in &value_map {
            if *count == 1 {
                return Some(Hand::TwoPair(TwoPairHand {
                    first_value: CardValue::new(pair_values[1]),
                    second_value: CardValue::new(pair_values[0]),
                    kicker: CardValue::new(*val),
                }));
            }
        }
    }

    None
}

fn get_pair(hand: CardCollection) -> Option<Hand> {
    let mut value_map = HashMap::new();
    for card in hand.0.clone() {
        let count = value_map.entry(card.value.value()).or_insert(0);
        *count += 1
    }

    for (val, count) in &value_map {
        if *count == 2 {
            let mut other_values = vec![];
            for card in hand.0.clone() {
                if card.value.value() != *val {
                    other_values.push(card.value.value());
                }
            }
            other_values.sort();
            return Some(Hand::Pair(PairHand {
                value: CardValue::new(*val),
                first_kicker: CardValue::new(other_values[2]),
                second_kicker: CardValue::new(other_values[1]),
                third_kicker: CardValue::new(other_values[0]),
            }));
        }
    }

    None
}

fn get_high_card(hand: CardCollection) -> Hand {
    let mut values = vec![];
    for card in hand.0 {
        values.push(card.value.value());
    }
    values.sort();

    Hand::HighCard(HighCardHand {
        first_kicker: CardValue::new(values[4]),
        second_kicker: CardValue::new(values[3]),
        third_kicker: CardValue::new(values[2]),
        fourth_kicker: CardValue::new(values[1]),
        fifth_kicker: CardValue::new(values[0]),
    })
}

fn straight_flush_tie_breaker(hands: Vec<StraightFlushHand>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![100; hands.len()];
    // sort hands by high card in descending order
    let mut hands_with_indices: Vec<_> = hands.iter().enumerate().collect();
    hands_with_indices.sort_by(|a, b| b.1.high_card.value().cmp(&a.1.high_card.value()));

    let mut rank = 1;
    for i in 0..hands_with_indices.len() - 1 {
        let cur = hands_with_indices[i];
        let next = hands_with_indices[i + 1];
        ret[cur.0] = rank;
        // only increase the rank if the next hand is not of equal strength
        if cur.1.high_card.value() != next.1.high_card.value() {
            rank += 1;
        }
    }
    ret[hands_with_indices[hands_with_indices.len() - 1].0] = rank;

    ret
}

fn four_of_a_kind_tie_breaker(hands: Vec<FourOfAKindHand>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![100; hands.len()];
    // sort hands by value of four of a kind in descending order
    let mut hands_with_indices: Vec<_> = hands.iter().enumerate().collect();
    hands_with_indices.sort_by(|a, b| b.1.value.value().cmp(&a.1.value.value()));

    let mut rank = 1;
    for i in 0..hands_with_indices.len() - 1 {
        let cur = hands_with_indices[i];
        ret[cur.0] = rank;
        // it's impossible for two four of a kinds to have the same strength, so we don't
        // need to do the extra check here to see if we have to increase rank
        rank += 1;
    }
    ret[hands_with_indices[hands_with_indices.len() - 1].0] = rank;

    ret
}

fn full_house_tie_breaker(hands: Vec<FullHouseHand>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![100; hands.len()];
    // sort hands by value of set of full house in descending order
    let mut hands_with_indices: Vec<_> = hands.iter().enumerate().collect();
    hands_with_indices.sort_by(|a, b| b.1.three_value.value().cmp(&a.1.three_value.value()));

    let mut rank = 1;
    for i in 0..hands_with_indices.len() - 1 {
        let cur = hands_with_indices[i];
        ret[cur.0] = rank;
        // it's impossible for two full houses to have the same strength, so we don't
        // need to do the extra check here to see if we have to increase rank
        rank += 1;
    }
    ret[hands_with_indices[hands_with_indices.len() - 1].0] = rank;

    ret
}

fn flush_tie_breaker(hands: Vec<FlushHand>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![100; hands.len()];
    // sort hands by high card in descending order
    let mut hands_with_indices: Vec<_> = hands.iter().enumerate().collect();
    hands_with_indices.sort_by(|a, b| b.1.high_card.value().cmp(&a.1.high_card.value()));

    let mut rank = 1;
    for i in 0..hands_with_indices.len() - 1 {
        let cur = hands_with_indices[i];
        let next = hands_with_indices[i + 1];
        ret[cur.0] = rank;
        // only increase the rank if the next hand is not of equal strength
        if cur.1.high_card.value() != next.1.high_card.value() {
            rank += 1;
        }
    }
    ret[hands_with_indices[hands_with_indices.len() - 1].0] = rank;

    ret
}

fn straight_tie_breaker(hands: Vec<StraightHand>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![100; hands.len()];
    // sort hands by high card in descending order
    let mut hands_with_indices: Vec<_> = hands.iter().enumerate().collect();
    hands_with_indices.sort_by(|a, b| b.1.high_card.value().cmp(&a.1.high_card.value()));

    let mut rank = 1;
    for i in 0..hands_with_indices.len() - 1 {
        let cur = hands_with_indices[i];
        let next = hands_with_indices[i + 1];
        ret[cur.0] = rank;
        // only increase the rank if the next hand is not of equal strength
        if cur.1.high_card.value() != next.1.high_card.value() {
            rank += 1;
        }
    }
    ret[hands_with_indices[hands_with_indices.len() - 1].0] = rank;

    ret
}

fn set_tie_breaker(hands: Vec<SetHand>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![100; hands.len()];
    // sort hands by value of set of full house in descending order
    let mut hands_with_indices: Vec<_> = hands.iter().enumerate().collect();
    hands_with_indices.sort_by(|a, b| b.1.value.value().cmp(&a.1.value.value()));

    let mut rank = 1;
    for i in 0..hands_with_indices.len() - 1 {
        let cur = hands_with_indices[i];
        ret[cur.0] = rank;
        // it's impossible for two sets to have the same strength, so we don't
        // need to do the extra check here to see if we have to increase rank
        rank += 1;
    }
    ret[hands_with_indices[hands_with_indices.len() - 1].0] = rank;

    ret
}

fn two_pair_tie_breaker(hands: Vec<TwoPairHand>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![100; hands.len()];
    // sort hands by value of highest pair
    // break ties by value of lower pair
    // break ties by value of kicker
    let mut hands_with_indices: Vec<_> = hands.iter().enumerate().collect();
    hands_with_indices.sort_by(|a, b| {
        b.1.first_value
            .value()
            .cmp(&a.1.first_value.value())
            .then_with(|| {
                b.1.second_value
                    .value()
                    .cmp(&a.1.second_value.value())
                    .then_with(|| b.1.kicker.value().cmp(&a.1.kicker.value()))
            })
    });

    let mut rank = 1;
    for i in 0..hands_with_indices.len() - 1 {
        let cur = hands_with_indices[i];
        let next = hands_with_indices[i + 1];
        ret[cur.0] = rank;
        // only increase the rank if the next hand is not of equal strength
        if cur.1.first_value.value() != next.1.first_value.value()
            || cur.1.second_value.value() != next.1.second_value.value()
            || cur.1.kicker.value() != next.1.kicker.value()
        {
            rank += 1;
        }
    }
    ret[hands_with_indices[hands_with_indices.len() - 1].0] = rank;

    ret
}

fn pair_tie_breaker(hands: Vec<PairHand>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![100; hands.len()];
    // sort hands by value of pair
    // break ties by value of highest kicker
    // break ties by value of next highest kicker
    // break ties by value of last kicker
    let mut hands_with_indices: Vec<_> = hands.iter().enumerate().collect();
    hands_with_indices.sort_by(|a, b| {
        b.1.value.value().cmp(&a.1.value.value()).then_with(|| {
            b.1.first_kicker
                .value()
                .cmp(&a.1.first_kicker.value())
                .then_with(|| {
                    b.1.second_kicker
                        .value()
                        .cmp(&a.1.second_kicker.value())
                        .then_with(|| b.1.third_kicker.value().cmp(&a.1.third_kicker.value()))
                })
        })
    });

    let mut rank = 1;
    for i in 0..hands_with_indices.len() - 1 {
        let cur = hands_with_indices[i];
        let next = hands_with_indices[i + 1];
        ret[cur.0] = rank;
        // only increase the rank if the next hand is not of equal strength
        if cur.1.value.value() != next.1.value.value()
            || cur.1.first_kicker.value() != next.1.first_kicker.value()
            || cur.1.second_kicker.value() != next.1.second_kicker.value()
            || cur.1.third_kicker.value() != next.1.third_kicker.value()
        {
            rank += 1;
        }
    }
    ret[hands_with_indices[hands_with_indices.len() - 1].0] = rank;

    ret
}

fn high_card_tie_breaker(hands: Vec<HighCardHand>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![100; hands.len()];
    // sort hands by value of high card
    // break ties by value of next highest kicker
    // break ties by value of next highest kicker
    // break ties by value of next highest kicker
    // break ties by value of next highest kicker
    let mut hands_with_indices: Vec<_> = hands.iter().enumerate().collect();
    hands_with_indices.sort_by(|a, b| {
        b.1.first_kicker
            .value()
            .cmp(&a.1.first_kicker.value())
            .then_with(|| {
                b.1.second_kicker
                    .value()
                    .cmp(&a.1.second_kicker.value())
                    .then_with(|| {
                        b.1.third_kicker
                            .value()
                            .cmp(&a.1.third_kicker.value())
                            .then_with(|| {
                                b.1.fourth_kicker
                                    .value()
                                    .cmp(&a.1.fourth_kicker.value())
                                    .then_with(|| {
                                        b.1.fifth_kicker.value().cmp(&a.1.fifth_kicker.value())
                                    })
                            })
                    })
            })
    });

    let mut rank = 1;
    for i in 0..hands_with_indices.len() - 1 {
        let cur = hands_with_indices[i];
        let next = hands_with_indices[i + 1];
        ret[cur.0] = rank;
        // only increase the rank if the next hand is not of equal strength
        if cur.1.first_kicker.value() != next.1.first_kicker.value()
            || cur.1.second_kicker.value() != next.1.second_kicker.value()
            || cur.1.third_kicker.value() != next.1.third_kicker.value()
            || cur.1.fourth_kicker.value() != next.1.fourth_kicker.value()
            || cur.1.fifth_kicker.value() != next.1.fifth_kicker.value()
        {
            rank += 1;
        }
    }
    ret[hands_with_indices[hands_with_indices.len() - 1].0] = rank;

    ret
}
