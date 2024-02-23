use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardSuit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy)]
pub enum CardValue {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl CardValue {
    pub fn new(val: u8) -> Self {
        match val {
            2 => CardValue::Two,
            3 => CardValue::Three,
            4 => CardValue::Four,
            5 => CardValue::Five,
            6 => CardValue::Six,
            7 => CardValue::Seven,
            8 => CardValue::Eight,
            9 => CardValue::Nine,
            10 => CardValue::Ten,
            11 => CardValue::Jack,
            12 => CardValue::Queen,
            13 => CardValue::King,
            14 => CardValue::Ace,
            _ => panic!(
                "Tried to create a card value with an invalid input value: {}",
                val
            ),
        }
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: CardSuit,
    pub value: CardValue,
}

pub struct Deck {
    cards: Vec<Card>,
    current_card: usize,
}

impl Deck {
    /// Creates a new deck of cards.
    pub fn new() -> Self {
        let suits: Vec<CardSuit> = vec![
            CardSuit::Heart,
            CardSuit::Diamond,
            CardSuit::Spade,
            CardSuit::Club,
        ];
        let values: Vec<CardValue> = vec![
            CardValue::Two,
            CardValue::Three,
            CardValue::Four,
            CardValue::Five,
            CardValue::Six,
            CardValue::Seven,
            CardValue::Eight,
            CardValue::Nine,
            CardValue::Ten,
            CardValue::Jack,
            CardValue::Queen,
            CardValue::King,
            CardValue::Ace,
        ];
        let mut cards: Vec<Card> = Vec::with_capacity(52);
        for suit in &suits {
            for value in &values {
                cards.push(Card {
                    suit: *suit,
                    value: *value,
                });
            }
        }
        Deck {
            cards,
            current_card: 0,
        }
    }

    /// Prints out all the cards in the deck in their current order.
    pub fn show(&self) {
        println!("{}", self.current_card);
        for card in &self.cards {
            println!("{:?}", card)
        }
    }

    /// Pops the first card off of the deck and returns it.
    pub fn pop_card(&mut self) -> Card {
        let cur = self.cards.get(self.current_card);
        match cur {
            Option::Some(card) => {
                self.current_card += 1;
                *card
            }
            None => {
                panic!("Tried to pop a card from the deck when no more exist")
            }
        }
    }

    /// Adds all cards back to the deck and shuffles them.
    pub fn reset(&mut self) {
        self.shuffle();
        self.current_card = 0;
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}
