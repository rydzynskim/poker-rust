use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum CardSuit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl fmt::Display for CardSuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CardSuit::Heart => {
                let heart = "\u{2665}".red();
                write!(f, "{}", heart)
            }
            CardSuit::Diamond => {
                let diamond = "\u{2666}".red();
                write!(f, "{}", diamond)
            }
            CardSuit::Spade => {
                write!(f, "\u{2660}")
            }
            CardSuit::Club => {
                write!(f, "\u{2663}")
            }
        }
    }
}

#[derive(Clone)]
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
        self.clone() as u8
    }
}

impl fmt::Display for CardValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CardValue::Two => {
                write!(f, "2")
            }
            CardValue::Three => {
                write!(f, "3")
            }
            CardValue::Four => {
                write!(f, "4")
            }
            CardValue::Five => {
                write!(f, "5")
            }
            CardValue::Six => {
                write!(f, "6")
            }
            CardValue::Seven => {
                write!(f, "7")
            }
            CardValue::Eight => {
                write!(f, "8")
            }
            CardValue::Nine => {
                write!(f, "9")
            }
            CardValue::Ten => {
                write!(f, "10")
            }
            CardValue::Jack => {
                write!(f, "J")
            }
            CardValue::Queen => {
                write!(f, "Q")
            }
            CardValue::King => {
                write!(f, "K")
            }
            CardValue::Ace => {
                write!(f, "A")
            }
        }
    }
}

#[derive(Clone)]
pub struct Card {
    pub suit: CardSuit,
    pub value: CardValue,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value.value() == 10 {
            writeln!(f, " ----- ")?;
            writeln!(f, "|     |")?;
            writeln!(f, "| {}{} |", self.value, self.suit)?;
            writeln!(f, "|     |")?;
            writeln!(f, " ----- ")?;
        } else {
            writeln!(f, " ----- ")?;
            writeln!(f, "|     |")?;
            writeln!(f, "| {}{}  |", self.value, self.suit)?;
            writeln!(f, "|     |")?;
            writeln!(f, " ----- ")?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct CardCollection(pub Vec<Card>);

impl CardCollection {
    pub fn concat(this: CardCollection, other: CardCollection) -> CardCollection {
        CardCollection([this.0, other.0].concat())
    }
}

impl fmt::Display for CardCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut top = String::new();
        let mut second = String::new();
        let mut third = String::new();
        let mut fourth = String::new();
        let mut bottom = String::new();

        for card in &self.0 {
            if card.value.value() == 10 {
                top.push_str(" -----  ");
                second.push_str("|     | ");
                third.push_str(&format!("| {}{} | ", card.value, card.suit));
                fourth.push_str("|     | ");
                bottom.push_str(" -----  ");
            } else {
                top.push_str(" -----  ");
                second.push_str("|     | ");
                third.push_str(&format!("| {}{}  | ", card.value, card.suit));
                fourth.push_str("|     | ");
                bottom.push_str(" -----  ");
            }
        }

        writeln!(f, "{}", top)?;
        writeln!(f, "{}", second)?;
        writeln!(f, "{}", third)?;
        writeln!(f, "{}", fourth)?;
        writeln!(f, "{}", bottom)?;

        Ok(())
    }
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
                    suit: suit.clone(),
                    value: value.clone(),
                });
            }
        }
        Deck {
            cards,
            current_card: 0,
        }
    }

    /// Pops the first card off of the deck and returns it.
    pub fn pop_cards(&mut self, num: u8) -> Option<CardCollection> {
        let mut cards: Vec<Card> = vec![];
        for _ in 0..num {
            let cur = self.cards.get(self.current_card)?;
            cards.push(cur.clone());
            self.current_card += 1;
        }

        Some(CardCollection(cards))
    }

    /// Adds all cards back to the deck and shuffles them.
    pub fn shuffle(&mut self) {
        self.current_card = 0;
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.cards {
            writeln!(f, "{}", card)?;
        }
        Ok(())
    }
}
