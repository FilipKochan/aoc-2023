use rust_base::aoc::Parser;

fn parse(line: String) -> Vec<String> {
    line.split(',').map(String::from).collect()
}

fn hash(data: &String) -> u32 {
    let mut cur = 0;

    for ch in data.chars() {
        cur += ch as u32;
        cur *= 17;
        cur %= 256;
    }

    cur
}

fn part1(instructions: &Vec<String>) -> u32 {
    instructions.iter().map(hash).sum()
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

fn part2(instructions: &Vec<String>) -> u32 {
    let mut map = vec![Vec::<Lens>::new(); 256];

    for instruction in instructions {
        let label;
        let mut fl = None;
        if instruction.contains('=') {
            let spl: Vec<&str> = instruction.split('=').collect();
            label = String::from(spl[0]);
            fl = spl[1].parse::<u32>().ok();
        } else {
            let mut cloned = instruction.clone();
            cloned.pop();
            label = cloned;
        }

        let h = hash(&label) as usize;
        if let Some(focal_length) = fl {
            if let Some(lens) = map[h]
                .iter()
                .enumerate()
                .find(|(_i, l)| l.label == label)
                .map(|v| v.0)
            {
                map[h][lens].focal_length = focal_length;
            } else {
                map[h].push(Lens {
                    label,
                    focal_length,
                });
            }
        } else {
            map[h].retain(|l| l.label != label);
        }
    }

    let mut res: u32 = 0;
    for (box_num, box_) in map.iter().enumerate() {
        for (lens_num, lens) in box_.iter().enumerate() {
            res += (box_num + 1) as u32 * (lens_num + 1) as u32 * lens.focal_length;
        }
    }
    res
}

fn main() {
    let parser = Parser::new();
    let bind = parser.parse_by_lines(parse);
    let instructions = bind.first().unwrap();
    println!("Part 1: {}", part1(instructions));
    println!("Part 2: {}", part2(instructions));
}
