use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<char>,
    bid: i32,
}

impl From<String> for Hand {
    fn from(value: String) -> Self {
        let splitted: Vec<&str> = value.split(' ').collect();
        let cards: Vec<char> = splitted[0].trim().chars().collect();
        let bid: i32 = splitted[1].trim().parse().unwrap();

        Self { cards, bid }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Combination {
    HighCard,
    One,
    Two,
    Three,
    FullHouse,
    Four,
    Five,
}

impl From<i32> for Combination {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::HighCard,
            2 => Self::One,
            3 => Self::Three,
            4 => Self::Two,
            5 => Self::Five,
            6 => Self::FullHouse,
            v => panic!("cannot convert from {}", v),
        }
    }
}

fn get_combination(card: HashMap<char, i32>) -> Combination {
    if card.values().any(|value| *value == 4) {
        return Combination::Four;
    }

    card.values()
        .fold(1, |acc, e| ((acc) * (*e)))
        .into()
}

fn occurences_map(cards: Vec<char>) -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();

    for card in cards {
        let c = *map.get(&card).unwrap_or(&0) + 1;
        map.insert(card, c);
    }

    map
}

fn card_value(card: char) -> i32 {
    match card {
        'A' => 100,
        'K' => 90,
        'Q' => 80,
        'J' => 70,
        'T' => 60,
        '*' => -1,
        v => v.to_digit(10).unwrap() as i32,
    }
}

fn compare_hands(left: &Hand, right: &Hand) -> std::cmp::Ordering {
    let comb_left = get_combination(occurences_map(left.cards.clone()));
    let comb_right = get_combination(occurences_map(right.cards.clone()));
    compare_combinations(
        comb_left,
        comb_right,
        left.cards.clone(),
        right.cards.clone(),
        false,
    )
}

fn compare_combinations(
    left: Combination,
    right: Combination,
    left_cards: Vec<char>,
    right_cards: Vec<char>,
    map_joker: bool,
) -> std::cmp::Ordering {
    if left < right {
        return std::cmp::Ordering::Less;
    }

    if left > right {
        return std::cmp::Ordering::Greater;
    }

    for (c1, c2) in left_cards.into_iter().zip(right_cards) {
        let mut c1_ = c1;
        let mut c2_ = c2;

        if map_joker && c1_ == 'J' {
            c1_ = '*';
        }
        if map_joker && c2_ == 'J' {
            c2_ = '*';
        }

        if card_value(c1_) < card_value(c2_) {
            return std::cmp::Ordering::Less;
        }

        if card_value(c1_) > card_value(c2_) {
            return std::cmp::Ordering::Greater;
        }
    }

    std::cmp::Ordering::Equal
}

fn find_best_combination(mut cards: Vec<char>) -> Combination {
    let different_cards: HashSet<char> = HashSet::from_iter(cards.clone());

    let next_joker = cards.iter().zip(0..).find(|(card, _)| **card == 'J');
    if let Some((_, index)) = next_joker {
        let mut best_combination = get_combination(occurences_map(cards.clone()));

        for possible in different_cards {
            if possible == 'J' {
                continue;
            }

            cards[index] = possible;
            let next_combination = find_best_combination(cards.clone());
            if next_combination > best_combination {
                best_combination = next_combination;
            }
        }

        cards[index] = 'J';
        return best_combination;
    }

    get_combination(occurences_map(cards))
}

fn compare_hands_joker(left: &Hand, right: &Hand) -> std::cmp::Ordering {
    let comb_left = find_best_combination(left.cards.clone());
    let comb_right = find_best_combination(right.cards.clone());

    compare_combinations(
        comb_left,
        comb_right,
        left.cards.clone(),
        right.cards.clone(),
        true,
    )
}

fn part(p: i32, mut hands: Vec<Hand>) -> i32 {
    if p == 1 {
        hands.sort_by(compare_hands);
    } else if p == 2 {
        hands.sort_by(compare_hands_joker);
    } else {
        panic!("unknown part {}", p);
    }

    hands.into_iter().zip(1..).map(|(h, ord)| h.bid * ord).sum()
}

fn main() -> io::Result<()> {
    if let Some(name) = args().nth(1) {
        let file = BufReader::new(File::open(name)?);
        let parsed_lines: Vec<Hand> = file.lines().flatten().map(|v| v.into()).collect();

        println!("Part 1: {}", part(1, parsed_lines.clone()));
        println!("Part 2: {}", part(2, parsed_lines));
    } else {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "input file not provided",
        ));
    }

    Ok(())
}
