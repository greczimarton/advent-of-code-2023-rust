use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut lines: Vec<Hand> = input.lines().map(|line| Hand::new(line)).collect();

    lines.sort();

    let mut sum = 0;

    for i in 0..lines.len() {
        sum += lines[i].bid * (i + 1) as u64;
    }

    sum.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
enum HandLabel {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    LetterT,
    LetterJ,
    LetterQ,
    LetterK,
    LetterA,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    lables: Vec<HandLabel>,
    hand_type: HandType,
    bid: u64,
}

impl Hand {
    fn new(line: &str) -> Self {
        let split: Vec<&str> = line.split(' ').collect();
        let lables = split.iter().nth(0).expect("should have labels");
        let bid = split
            .iter()
            .nth(1)
            .expect("should have bid")
            .parse::<u64>()
            .expect("should parse bid to u64");

        Hand {
            lables: parse_line_to_hand_labels(lables),
            hand_type: parse_line_to_hand_type(lables),
            bid: bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.hand_type.cmp(&other.hand_type)

        if self.hand_type < other.hand_type {
            return Ordering::Less;
        } else if self.hand_type > other.hand_type {
            return Ordering::Greater;
        } else {
            for i in 0..5 {
                if self.lables[i] < other.lables[i] {
                    return Ordering::Less;
                } else if self.lables[i] > other.lables[i] {
                    return Ordering::Greater;
                }
            }
            return Ordering::Equal;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line_to_hand_type(line: &str) -> HandType {
    let mut hash_map: HashMap<char, u32> = HashMap::new();
    let chars: Vec<char> = line.chars().collect();

    for c in chars {
        if hash_map.contains_key(&c) {
            let count: &mut u32 = hash_map.get_mut(&c).expect("should have char");
            *count += 1;
        } else {
            hash_map.insert(c, 1);
        }
    }

    let values: Vec<u32> = hash_map.values().cloned().collect();

    if values.iter().any(|count| *count == 5) {
        return HandType::FiveOfAKind;
    }

    if values.iter().any(|count| *count == 4) {
        return HandType::FourOfAKind;
    }

    if values.iter().any(|count| *count == 3) && values.iter().any(|count| *count == 2) {
        return HandType::FullHouse;
    }

    if values.iter().any(|count| *count == 3) {
        return HandType::ThreeOfAKind;
    }

    let pair_count = values.iter().filter(|count| **count == 2).count();

    if pair_count == 2 {
        return HandType::TwoPair;
    }

    if pair_count == 1 {
        return HandType::OnePair;
    }
    let unique_values: Vec<char> = hash_map.keys().into_iter().unique().cloned().collect();
    if unique_values.len() == 5 {
        return HandType::HighCard;
    }

    panic!("invalid hand type");
}

fn parse_line_to_hand_labels(line: &str) -> Vec<HandLabel> {
    let mut labels = vec![];

    let chars: Vec<char> = line.chars().collect();

    for character in chars {
        match character {
            'A' => labels.push(HandLabel::LetterA),
            'K' => labels.push(HandLabel::LetterK),
            'Q' => labels.push(HandLabel::LetterQ),
            'J' => labels.push(HandLabel::LetterJ),
            'T' => labels.push(HandLabel::LetterT),
            '9' => labels.push(HandLabel::Nine),
            '8' => labels.push(HandLabel::Eight),
            '7' => labels.push(HandLabel::Seven),
            '6' => labels.push(HandLabel::Six),
            '5' => labels.push(HandLabel::Five),
            '4' => labels.push(HandLabel::Four),
            '3' => labels.push(HandLabel::Three),
            '2' => labels.push(HandLabel::Two),
            '1' => labels.push(HandLabel::One),
            _ => panic!("invalid label"),
        }
    }

    return labels;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_high_card_correctly() {
        let hand_type = parse_line_to_hand_type("23456");
        assert_eq!(hand_type, HandType::HighCard)
    }
    #[test]
    fn parse_one_pair_correctly() {
        let hand_type = parse_line_to_hand_type("A23A4");
        assert_eq!(hand_type, HandType::OnePair)
    }
    #[test]
    fn parse_two_pair_correctly() {
        let hand_type = parse_line_to_hand_type("23432");
        assert_eq!(hand_type, HandType::TwoPair)
    }
    #[test]
    fn parse_three_of_a_kind_correctly() {
        let hand_type = parse_line_to_hand_type("TTT98");
        assert_eq!(hand_type, HandType::ThreeOfAKind)
    }
    #[test]
    fn parse_full_house_correctly() {
        let hand_type = parse_line_to_hand_type("23332");
        assert_eq!(hand_type, HandType::FullHouse)
    }
    #[test]
    fn parse_four_of_a_kind_correctly() {
        let hand_type = parse_line_to_hand_type("AA8AA");
        assert_eq!(hand_type, HandType::FourOfAKind)
    }
    #[test]
    fn parse_five_of_a_kind_correctly() {
        let hand_type = parse_line_to_hand_type("AAAAA");
        assert_eq!(hand_type, HandType::FiveOfAKind)
    }

    #[test]
    fn it_works() {
        let input = include_str!("./input_test.txt");
        let result = process(input);
        assert_eq!(result, "6440");
    }
}
