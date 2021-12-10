use std::collections::HashMap;

struct CrabData {
    min_pos: usize,
    max_pos: usize,
    total: usize,
    crab_positions: HashMap<usize, usize>,
}

#[derive(Debug)]
struct Answer {
    pos: usize,
    min_fuel: usize,
}

pub fn part1(input: String) {
    let crab_data = parse_crabs(&input);
    let mut behind = 0;
    let mut at = crab_data
        .crab_positions
        .get(&crab_data.min_pos)
        .map(|p| *p)
        .unwrap();
    let mut ahead = crab_data.total - at;

    let mut at_fuel = crab_data
        .crab_positions
        .iter()
        .fold(0, |acc, (key, val)| acc + key * val);
    let mut answer = Answer {
        pos: 0,
        min_fuel: at_fuel,
    };

    for i in crab_data.min_pos + 1..crab_data.max_pos + 1 {
        at_fuel += behind;
        at_fuel += at;
        at_fuel -= ahead;

        if at_fuel < answer.min_fuel {
            answer.pos = i;
            answer.min_fuel = at_fuel;
        }

        behind += at;
        at = crab_data.crab_positions.get(&i).map(|p| *p).unwrap_or(0);
        ahead -= at;
    }
    println!("{:?}", answer);
}

fn parse_crabs(input: &str) -> CrabData {
    let mut crab_positions = HashMap::new();
    let mut min_pos = None;
    let mut max_pos = None;
    let mut total = 0;

    input.trim().split(",").for_each(|p| {
        total += 1;
        let p = p.parse::<usize>().unwrap();
        match min_pos {
            None => min_pos = Some(p),
            Some(k) => {
                if p < k {
                    min_pos = Some(p)
                }
            }
        }
        match max_pos {
            None => max_pos = Some(p),
            Some(k) => {
                if p > k {
                    max_pos = Some(p)
                }
            }
        }
        match crab_positions.get_mut(&p) {
            None => {
                crab_positions.insert(p, 1);
            }
            Some(k) => *k += 1,
        };
    });
    CrabData {
        min_pos: min_pos.unwrap(),
        max_pos: max_pos.unwrap(),
        total,
        crab_positions,
    }
}
