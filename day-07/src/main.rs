use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn load_file_in_memory(filepath: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        data.push(line.unwrap());
    }

    Ok(data)
}


mod part_01 {
    use crate::load_file_in_memory;
    use std::{cmp::Ordering, collections::HashMap};

    #[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
    enum Rank {
        HighCard,
        OnePair,
        TwoPairs,
        ThreeOfAKind,
        Full,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

    #[derive(PartialEq, Eq, Debug)]
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
            /* Sort ascending... */
            values.sort();
            /* and reverse ot to get values in descending order */
            values.reverse();
            match values[0] {
                5 => return Some(Rank::FiveOfAKind),
                4 => return Some(Rank::FourOfAKind),
                3 => { if values[1] == 2 { return Some(Rank::Full) } else { return Some(Rank::ThreeOfAKind) } },
                2 => { if values[1] == 2 { return Some(Rank::TwoPairs) } else { return Some(Rank::OnePair) } },
                1 => return Some(Rank::HighCard),
                _ => return None,
            }
        }

        fn from_str(string: &str, bid: u64) -> Hand {
            Hand::from(&(string.chars().collect()), bid)
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
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }


    fn transform_data(data: Vec<String>) -> Vec<Hand> {
        let mut hands = Vec::new();

        for line in data {
            let mut parsed_data = line.split(" ");
            match (parsed_data.next(), parsed_data.next()) {
                (Some(cards), Some(bid)) => hands.push(Hand::from_str(cards, bid.parse().unwrap())),
                (_, _) => {},
            }
        }

        hands
    }

    pub fn resolve() {
        let data = load_file_in_memory("./input-01.data").unwrap();
        let mut hands = transform_data(data);
        let mut rank = 1;
        hands.sort();
        for hand in hands.iter_mut() {
            hand.rank = rank;
            println!("{:?}", hand);
            rank += 1;
        }
        let final_result: u64 = hands.iter().map(|hand| hand.rank * hand.bid).sum();

        println!("Part 1 final result: {}", final_result);
    }
}

mod part_02 {
    use crate::load_file_in_memory;
    use std::{cmp::Ordering, collections::{HashMap}};

    #[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
    enum Rank {
        HighCard,
        OnePair,
        TwoPairs,
        ThreeOfAKind,
        Full,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, Debug)]
    enum Symbols {
        Joker,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
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
                'T' => Some(Symbols::Ten),
                '9' => Some(Symbols::Nine),
                '8' => Some(Symbols::Eight),
                '7' => Some(Symbols::Seven),
                '6' => Some(Symbols::Six),
                '5' => Some(Symbols::Five),
                '4' => Some(Symbols::Four),
                '3' => Some(Symbols::Three),
                '2' => Some(Symbols::Two),
                'J' => Some(Symbols::Joker),
                _ => None,
            }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
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

            Hand::process_jokers_in_hand(&mut hashmap);

            let mut values: Vec<u32> = hashmap.into_values().collect();
            /* Sort ascending... */
            values.sort();
            /* and reverse ot to get values in descending order */
            values.reverse();
            match values[0] {
                5 => return Some(Rank::FiveOfAKind),
                4 => return Some(Rank::FourOfAKind),
                3 => { if values[1] == 2 { return Some(Rank::Full) } else { return Some(Rank::ThreeOfAKind) } },
                2 => { if values[1] == 2 { return Some(Rank::TwoPairs) } else { return Some(Rank::OnePair) } },
                1 => return Some(Rank::HighCard),
                _ => return None,
            }
        }

        fn process_jokers_in_hand(hand: &mut HashMap<Symbols, u32>) {
            let jokers = hand.get(&Symbols::Joker);
            let joker_numbers;
            match jokers {
                None => return,
                Some(number) => { joker_numbers = *number },
            }

            let most = Hand::find_most_occuring_concrete_symbol_in_hand(hand);
            match most {
                None => return,
                Some(symbol) => { hand.entry(*symbol).and_modify(|occurences| *occurences += joker_numbers); },
            }
            hand.entry(Symbols::Joker).and_modify(|number| *number = 0);
        }

        fn find_most_occuring_concrete_symbol_in_hand(hand: &HashMap<Symbols, u32>) -> Option<&Symbols> {
            let mut most_occuring = None;

            for card in hand.keys() {
                if card == &Symbols::Joker { continue; }

                let destructured_card = hand.get_key_value(card).unwrap();
                match most_occuring {
                    None => most_occuring = Some(destructured_card),
                    Some((_, max)) => {
                        if destructured_card.1 > max {
                            most_occuring = Some(destructured_card);
                        }
                    },
                }
            }

            match most_occuring {
                None => None,
                Some((symbol, _)) => Some(symbol),
            }
        }

        fn from_str(string: &str, bid: u64) -> Hand {
            Hand::from(&(string.chars().collect()), bid)
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
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }


    fn transform_data(data: Vec<String>) -> Vec<Hand> {
        let mut hands = Vec::new();

        for line in data {
            let mut parsed_data = line.split(" ");
            match (parsed_data.next(), parsed_data.next()) {
                (Some(cards), Some(bid)) => hands.push(Hand::from_str(cards, bid.parse().unwrap())),
                (_, _) => {},
            }
        }

        hands
    }

    pub fn resolve() {
        let data = load_file_in_memory("./input-02.data").unwrap();
        let mut hands = transform_data(data);
        let mut rank = 1;
        hands.sort();
        for hand in hands.iter_mut() {
            hand.rank = rank;
            println!("{:?}", hand);
            rank += 1;
        }
        let final_result: u64 = hands.iter().map(|hand| hand.rank * hand.bid).sum();

        println!("Part 2 final result: {}", final_result);
    }
}

fn main() {
    let now = Instant::now();
    part_01::resolve();
    let elapsed = now.elapsed();
    println!("Part 1 found in {:?}s", elapsed.as_secs());
    let now = Instant::now();
    part_02::resolve();
    let elapsed = now.elapsed();
    println!("Part 2 found in {:?}s", elapsed.as_secs());
}
