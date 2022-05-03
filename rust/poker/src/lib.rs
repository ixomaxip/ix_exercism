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
            _ => unreachable!("Invalid card value")
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

#[derive(Debug)]
enum Suit {
    DIAMONDS, CLUBS, HEARTS, SPADES
}

#[derive(Debug)]
struct Card {
    value: Value,
    suit: Suit,
}

#[derive(Debug)]
struct PokerHand {
    cards: [Card; 5]
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
