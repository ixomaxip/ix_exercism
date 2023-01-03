use std::{
    collections::HashMap,
    cmp::Ordering,
    fmt
};
use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;

#[repr(usize)]
#[derive(Debug, Eq, Clone, Copy, IntEnum, Hash)]
enum Value {
    Ace = 1,
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
}

impl Value {
    fn new(value: &str) -> Value {
        match value {
            "A" => Value::Ace,
            "2" => Value::Two,
            "3" => Value::Three,
            "4" => Value::Four,
            "5" => Value::Five,
            "6" => Value::Six,
            "7" => Value::Seven,
            "8" => Value::Eight,
            "9" => Value::Nine,
            "10" => Value::Ten,
            "J" => Value::Jack,
            "Q" => Value::Queen,
            "K" => Value::King,
            _ => unreachable!("{}", format!("Invalid card value: {}", value)),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.int_value()).cmp(&(other.int_value()))
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.int_value()).cmp(&(other.int_value())))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        *self as i32 == *other as i32
    }
}

#[repr(usize)]
#[derive(Debug, Eq, Clone, Copy, IntEnum)]
enum Suit {
    Diamonds = 1,
    Clubs = 2,
    Hearts = 3,
    Spades = 4,
}

impl Suit {
    fn new(suit: &str) -> Suit {
        match suit {
            "D" => Suit::Diamonds,
            "C" => Suit::Clubs,
            "H" => Suit::Hearts,
            "S" => Suit::Spades,
            _ => unreachable!("{}", format!("Invalid suit: {}", suit)),
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
            suit: Suit::new(&suit),
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

#[repr(usize)]
#[derive(Debug, Eq, Clone, Copy, IntoEnumIterator, IntEnum)]
enum Rank {
    NA = 0,
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourKind = 8,
    StraightFlush = 9,
    FiveKind = 10,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.int_value()).cmp(&(other.int_value()))
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.int_value()).cmp(&(other.int_value())))
    }
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self.int_value() == other.int_value()
    }
}

#[derive(Debug)]
struct PokerHand {
    cards: [Card; 5],
    rank: Rank,
}

impl PokerHand {
    fn from_string(hand: &str) -> Self {
        let mut cards = hand
            .split(" ")
            .map(|c| Card::from_str(c))
            .collect::<Vec<_>>();
        cards.sort_by(|a, b| a.cmp(b));
        let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];
        PokerHand {
            cards: cards,
            rank: Self::calc_rank(&cards),
        }
    }

    fn calc_rank(cards: &[Card]) -> Rank {

        let counters: HashMap<Value, usize> = cards
            .iter()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(c.value).or_insert(0) += 1;
                map
            });
            println!("{:?}", counters);
            if Self::is_FiveKind(&counters) {
                Rank::FiveKind
            } else if Self::is_FourKind(&counters) {
                Rank::FourKind
            } else if Self::is_ThreeKind(&counters) {
                Rank::ThreeKind
            } else if Self::is_TwoPair(&counters) {
                Rank::TwoPair
            } else if Self::is_OnePair(&counters) {
                Rank::OnePair
            } else {
                Rank::HighCard
            }

    }

    fn find_value(map: &HashMap<Value, usize>, value: usize) -> usize {
        map.values().fold(0, |acc, &v| if v == value {acc + 1 } else { acc })
    }

    fn is_FiveKind(counters: &HashMap<Value, usize>) -> bool {
        Self::find_value(counters, 5) == 1
    }

    fn is_FourKind(counters: &HashMap<Value, usize>) -> bool {
        Self::find_value(counters, 4) == 1
    }

    fn is_ThreeKind(counters: &HashMap<Value, usize>) -> bool {
        Self::find_value(counters, 3) == 1
    }

    fn is_TwoPair(counters: &HashMap<Value, usize>) -> bool {
        Self::find_value(counters, 2) == 2
    }

    fn is_OnePair(counters: &HashMap<Value, usize>) -> bool {
        Self::find_value(counters, 2) == 1
    }
}

impl fmt::Display for PokerHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards = self.cards
            .iter()
            .map(|c| format!("{:?} - {:?}", c.value, c.suit))
            .collect::<Vec<_>>();

        write!(f, "rank {:?}: {:?}", self.rank, cards.join(" | "))
    }
}
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut result_hands: Vec<&str> = vec![];

    let t_hands = hands
        .iter()
        .map(|h| PokerHand::from_string(h))
        .collect::<Vec<_>>();

    for h in t_hands {
        println!("hand {:}", h);
    }

    for h in hands {
        result_hands.push(h)
    }

    result_hands
}
