use std::{
    collections::{HashSet, HashMap},
    env::args,
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

fn parse_cards(line: &str) -> HashSet<i32> {
    line.split(' ')
        .flat_map(|v| v.parse::<i32>())
        .collect()
}

fn parse_line(line: String) -> (i64, HashSet<i32>, HashSet<i32>) {
    let splitted: Vec<&str> = line.split(':').collect();
    let card = splitted[0];
    let card_id: i64 =card.split(' ').last().unwrap().parse().unwrap();
    let cards: Vec<&str> = splitted[1].split('|').collect();
    let my_cards: HashSet<i32> = parse_cards(cards[0]);
    let winning_cards: HashSet<i32> = parse_cards(cards[1]);

    (card_id, my_cards, winning_cards)
}

fn score(from_line: (i64, HashSet<i32>, HashSet<i32>)) -> i64 {
    let (_, my_cards, winning_cards) = from_line;
    let same: u32 = my_cards.intersection(&winning_cards).count() as u32;
    if same == 0 {
        return 0;
    };
    let res: i64 = 2i64.pow(same - 1);
    res
}

fn main() -> io::Result<()> {
    let name = args().nth(1);

    if let Some(name) = name {
        let file = BufReader::new(File::open(name)?);
        let parsed_lines: Vec<(i64, HashSet<i32>, HashSet<i32>)> = file.lines().flatten().map(parse_line).collect();
        let part_1_res: i64 = parsed_lines.clone().into_iter().map(score).sum();
        
        let mut costs: HashMap<i64, i64> = HashMap::new();

        let part_2_res: i64 = parsed_lines.into_iter().rev().map(|(id, my_cards, winning_cards)| {
            let winning_numbers: i64 = my_cards.intersection(&winning_cards).count() as i64; 
            costs.insert(id, 1);
            for i in 1..=winning_numbers {
                let key = id + i;
                if costs.contains_key(&key) {
                    let cost = costs[&key];
                    costs.get_mut(&id).map(|value| *value += cost);
                }
            }
            costs[&id]
        }).sum();

        println!("Part 1: {}", part_1_res);
        println!("Part 2: {}", part_2_res);
    } else {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "input file not provided",
        ));
    }

    Ok(())
}
