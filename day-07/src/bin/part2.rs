use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

//244768107 BAD
//244768107 BAD
//244757651 BAD
//244757651 BAD
//244848487 ???

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut lines: Vec<Hand> = input.lines().map(|line| Hand::new(line)).collect();

    lines.sort();

    dbg!(&lines);

    let mut sum = 0;

    for i in 0..lines.len() {
        sum += lines[i].bid * (i + 1) as u64;
    }

    sum.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_num = *self as u64;
        let other_num = *other as u64;

        return self_num.cmp(&other_num);
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum HandLabel {
    LetterJ,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    LetterT,
    LetterQ,
    LetterK,
    LetterA,
}

impl PartialOrd for HandLabel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandLabel {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_num = *self as u64;
        let other_num = *other as u64;

        return self_num.cmp(&other_num);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    text: String,
    labels: Vec<HandLabel>,
    hand_type: HandType,
    bid: u64,
}

impl Hand {
    fn new(line: &str) -> Self {
        let split: Vec<&str> = line.split(' ').collect();
        let enum_labels: Vec<HandLabel> =
            parse_line_to_hand_labels(split.iter().nth(0).expect("should have labels"));
        let bid = split
            .iter()
            .nth(1)
            .expect("should have bid")
            .parse::<u64>()
            .expect("should parse bid to u64");

        Hand {
            text: line.to_string(),
            labels: enum_labels.clone(),
            hand_type: parse_line_to_hand_type(&enum_labels),
            bid: bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.hand_type.cmp(&other.hand_type);

        if ordering != Ordering::Equal {
            return ordering;
        }

        for i in 0..5 {
            if self.labels[i] < other.labels[i] {
                return Ordering::Less;
            } else if self.labels[i] > other.labels[i] {
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line_to_hand_type(labels: &Vec<HandLabel>) -> HandType {
    let mut hash_map: HashMap<HandLabel, u32> = HashMap::new();

    for label in labels {
        if hash_map.contains_key(&label) {
            let count: &mut u32 = hash_map.get_mut(&label).expect("should have char");
            *count += 1;
        } else {
            hash_map.insert(label.clone(), 1);
        }
    }

    let values: Vec<u32> = hash_map.values().cloned().collect();

    if values.iter().any(|count| *count == 5) {
        return HandType::FiveOfAKind;
    }

    if values.iter().any(|count| *count == 4) {
        if hash_map.contains_key(&HandLabel::LetterJ) {
            return HandType::FiveOfAKind;
        }

        return HandType::FourOfAKind;
    }

    if values.iter().any(|count| *count == 3) && values.iter().any(|count| *count == 2) {
        if hash_map.contains_key(&HandLabel::LetterJ) {
            return HandType::FiveOfAKind;
        }

        return HandType::FullHouse;
    }

    if values.iter().any(|count| *count == 3) {
        if hash_map.contains_key(&HandLabel::LetterJ) {
            if hash_map[&HandLabel::LetterJ] == 1 {
                return HandType::FourOfAKind;
            }
            if hash_map[&HandLabel::LetterJ] == 2 {
                return HandType::FiveOfAKind;
            }
            if hash_map[&HandLabel::LetterJ] == 3 {
                return HandType::FourOfAKind;
            }
        }

        return HandType::ThreeOfAKind;
    }

    let pair_count = values.iter().filter(|count| **count == 2).count();

    if pair_count == 2 {
        if hash_map.contains_key(&HandLabel::LetterJ) {
            if hash_map[&HandLabel::LetterJ] == 1 {
                return HandType::FullHouse;
            }
            if hash_map[&HandLabel::LetterJ] == 2 {
                return HandType::FourOfAKind;
            }
        }

        return HandType::TwoPair;
    }

    if pair_count == 1 {
        if hash_map.contains_key(&HandLabel::LetterJ) {
            if hash_map[&HandLabel::LetterJ] == 1 {
                return HandType::ThreeOfAKind;
            }
            if hash_map[&HandLabel::LetterJ] == 2 {
                return HandType::ThreeOfAKind;
            }
            if hash_map[&HandLabel::LetterJ] == 3 {
                return HandType::FiveOfAKind;
            }
        }
        return HandType::OnePair;
    }
    let unique_values: Vec<HandLabel> = hash_map.keys().into_iter().unique().cloned().collect();
    if unique_values.len() == 5 {
        if hash_map.contains_key(&HandLabel::LetterJ) {
            return HandType::OnePair;
        }
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
            _ => panic!("invalid label"),
        }
    }

    return labels;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn ajj93() {
    //     let input_vec = parse_line_to_hand_labels("ajj93");
    //     let hand_type = parse_line_to_hand_type(&input_vec);
    //     assert_eq!(hand_type, HandType::ThreeOfAKind)
    // }
    // #[test]
    // fn parse_one_pair_correctly() {
    //     let hand_type = parse_line_to_hand_type("A23A4");
    //     assert_eq!(hand_type, HandType::OnePair)
    // }
    // #[test]
    // fn parse_two_pair_correctly() {
    //     let hand_type = parse_line_to_hand_type("23432");
    //     assert_eq!(hand_type, HandType::TwoPair)
    // }
    // #[test]
    // fn parse_three_of_a_kind_correctly() {
    //     let hand_type = parse_line_to_hand_type("TTT98");
    //     assert_eq!(hand_type, HandType::ThreeOfAKind)
    // }
    // #[test]
    // fn parse_full_house_correctly() {
    //     let hand_type = parse_line_to_hand_type("23332");
    //     assert_eq!(hand_type, HandType::FullHouse)
    // }
    // #[test]
    // fn parse_four_of_a_kind_correctly() {
    //     let hand_type = parse_line_to_hand_type("AA8AA");
    //     assert_eq!(hand_type, HandType::FourOfAKind)
    // }
    // #[test]
    // fn parse_five_of_a_kind_correctly() {
    //     let hand_type = parse_line_to_hand_type("AAAAA");
    //     assert_eq!(hand_type, HandType::FiveOfAKind)
    // }

    #[test]
    fn it_works() {
        let input = include_str!("./input_test.txt");
        let result = process(input);
        assert_eq!(result, "5905");
    }
}
