use std::cmp::Ordering;

#[derive(Debug, Eq, Clone, Copy)]
enum Value {
    ACE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, QUEEN, KING
}

impl Value {
    fn new(value: &str) -> Value {
        match value {
            "A" => Value::ACE,
            "2" => Value::TWO,
            "3" => Value::THREE,
            "4" => Value::FOUR,
            "5" => Value::FIVE,
            "6" => Value::SIX,
            "7" => Value::SEVEN,
            "8" => Value::EIGHT,
            "9" => Value::NINE,
            "10" => Value::TEN,
            "J" => Value::JACK,
            "Q" => Value::QUEEN,
            "K" => Value::KING,
            _ => unreachable!("{}", format!("Invalid card value: {}", value))
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((*self as i32).cmp(&(*other as i32)))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        *self as i32 == *other as i32
    }
}

#[derive(Debug, Eq, Clone, Copy)]
enum Suit {
    DIAMONDS, CLUBS, HEARTS, SPADES
}

impl Suit {
    fn new(suit: &str) -> Suit {
        match suit {
            "D" => Suit::DIAMONDS,
            "C" => Suit::CLUBS,
            "H" => Suit::HEARTS,
            "S" => Suit::SPADES,
            _ => unreachable!("{}", format!("Invalid suit: {}", suit))
        }
    }
}

impl PartialEq for Suit {
    fn eq(&self, other: &Self) -> bool {
        *self as i32 == *other as i32
    }
}

#[derive(Debug, Eq, Clone, Copy)]
struct Card {
    value: Value,
    suit: Suit,
}

impl Card {
    fn from_str(card: &str) -> Self {
        let suit = card.chars().last().unwrap().to_string();
        let mut chars = card.chars();
        chars.next_back();
        let val = chars.as_str();
        Card {
            value: Value::new(&val),
            suit: Suit::new(&suit)
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Debug)]
struct PokerHand {
    cards: [Card; 5]
}

impl PokerHand {
    fn from_string(hand: &str) -> Self {
        let mut cards = hand
            .split(" ")
            .map(|c| Card::from_str(c))
            .collect::<Vec<_>>();
        cards.sort();
        PokerHand {
            cards: [
                cards[0],
                cards[1],
                cards[2],
                cards[3],
                cards[4]
            ]
        }
    }
}
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let result_hands: Vec<&str> = vec![];

    let t_hands = hands
        .iter()
        .map(|h| PokerHand::from_string(h))
        .collect::<Vec<_>>();

    println!("{:?}", t_hands);

    result_hands
}
