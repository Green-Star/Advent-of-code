use std::{cmp::Ordering, collections::HashMap};

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum Rank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    Full,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
enum Symbols {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Symbols {
    fn from(c: &char) -> Option<Symbols> {
        match c {
            'A' => Some(Symbols::Ace),
            'K' => Some(Symbols::King),
            'Q' => Some(Symbols::Queen),
            'J' => Some(Symbols::Jack),
            'T' => Some(Symbols::Ten),
            '9' => Some(Symbols::Nine),
            '8' => Some(Symbols::Eight),
            '7' => Some(Symbols::Seven),
            '6' => Some(Symbols::Six),
            '5' => Some(Symbols::Five),
            '4' => Some(Symbols::Four),
            '3' => Some(Symbols::Three),
            '2' => Some(Symbols::Two),
            _ => None,
        }
    }
}

#[derive(PartialOrd, PartialEq, Eq)]
struct Hand {
    cards: Vec<Symbols>,
    bid: u64,
    strength: Rank,
    rank: u64,
}
impl Hand {
    fn from(hand: &Vec<char>, bid: u64) -> Hand {
        let mut cards = Vec::new();
        for c in hand {
            cards.push(Symbols::from(c).unwrap());
        }
        let strength = Hand::get_hand_strength(&cards).unwrap();

        Hand { cards, bid, strength, rank: 0 }
    }

    fn get_hand_strength(cards: &Vec<Symbols>) -> Option<Rank> {
        let mut hashmap: HashMap<Symbols, u32> = HashMap::new();
        for c in cards {
            hashmap.entry(*c).and_modify(|number| *number += 1).or_insert(1);
        }

        let mut values: Vec<u32> = hashmap.into_values().collect();
        values.sort();
        match values[0] {
            5 => return Some(Rank::FiveOfAKind),
            4 => return Some(Rank::FourOfAKind),
            3 => { if values[1] == 2 { return Some(Rank::Full) } else { return Some(Rank::ThreeOfAKind) } },
            2 => { if values[1] == 2 { return Some(Rank::TwoPairs) } else { return Some(Rank::OnePair) } },
            1 => return Some(Rank::HighCard),
            _ => return None,
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength > other.strength {
            return Ordering::Greater
        } else if self.strength < other.strength {
            return Ordering::Less
        }
        let mut self_cards = self.cards.iter();
        let mut other_cards = other.cards.iter();
        while let (Some(our_card), Some(their_card)) = (self_cards.next(), other_cards.next()) {
            if our_card > their_card {
                return Ordering::Greater
            } else if our_card < their_card {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}


fn main() {
    assert!(Rank::FiveOfAKind > Rank::FourOfAKind);
    assert!(Rank::ThreeOfAKind > Rank::TwoPairs);
    assert!(Rank::ThreeOfAKind > Rank::HighCard);

    assert!(Symbols::Ace > Symbols::King);
    assert!(Symbols::Ten > Symbols::Two);
    assert!(Symbols::Eight < Symbols::Jack);



    let left: Hand = Hand { strength: Rank::FourOfAKind, cards: vec![
        Symbols::Three, Symbols::Three, Symbols::Three, Symbols::Three, Symbols::Two
    ], bid: 0, rank: 0 };
    let right: Hand = Hand { strength: Rank::FourOfAKind, cards: vec![
        Symbols::Two, Symbols::Ace, Symbols::Ace, Symbols::Ace, Symbols::Ace
    ], bid: 0, rank: 0 };
    assert!(left > right);

    let left: Hand = Hand { strength: Rank::Full, cards: vec![
        Symbols::Seven, Symbols::Seven, Symbols::Eight, Symbols::Eight, Symbols::Eight
    ], bid: 0, rank: 0 };
    let right: Hand = Hand { strength: Rank::Full, cards: vec![
        Symbols::Seven, Symbols::Seven, Symbols::Seven, Symbols::Eight, Symbols::Eight
    ], bid: 0, rank: 0 };
    assert!(left > right);

    let one: Hand = Hand { strength: Rank::OnePair, cards: vec![
        Symbols::Three, Symbols::Two, Symbols::Ten, Symbols::Three, Symbols::King
    ], bid: 0, rank: 0 };
    let two: Hand = Hand { strength: Rank::TwoPairs, cards: vec![
        Symbols::King, Symbols::King, Symbols::Six, Symbols::Seven, Symbols::Seven
    ], bid: 0, rank: 0 };
    let three: Hand = Hand { strength: Rank::TwoPairs, cards: vec![
        Symbols::King, Symbols::Ten, Symbols::Jack, Symbols::Jack, Symbols::Ten
    ], bid: 0, rank: 0 };
    assert!(one < two);
    assert!(one < three);
    assert!(two > three);

    println!("Hello, world!");
}
