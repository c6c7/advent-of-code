use {
    std::convert::TryInto,
    tracing::{debug, info},
};

const NUM_STACKS: usize = 9;

fn read_stacks(input: &str) -> [Vec<char>; NUM_STACKS] {
    let mut stacks: [Vec<char>; NUM_STACKS] = Default::default();
    for line in input.split("\n") {
        let line_bytes = line.as_bytes();
        for i in 0..NUM_STACKS {
            let idx = 1 + i * 4;
            if line_bytes[idx] == b' ' {
                continue;
            }
            stacks[i].push(line_bytes[idx].try_into().unwrap())
        }
    }
    for stack in &mut stacks {
        stack.pop(); // Remove the number from the bottom
        stack.reverse();
    }
    stacks
}

#[derive(Debug)]
struct CrateMove {
    num_crates: usize,
    from: usize,
    to: usize,
}

fn read_moves(input: &str) -> Vec<CrateMove> {
    let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut moves = Vec::new();
    for cap in re.captures_iter(&input) {
        let cap = cap
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        moves.push(CrateMove {
            num_crates: cap[0],
            from: cap[1] - 1,
            to: cap[2] - 1,
        });
    }
    moves
}

fn move_crates_9000(stacks: &mut [Vec<char>; NUM_STACKS], moves: Vec<CrateMove>) {
    for m in moves {
        let mut crate_buffer = Vec::new();
        for _ in 0..m.num_crates {
            crate_buffer.push(stacks[m.from].pop().unwrap());
        }
        for i in 0..m.num_crates {
            stacks[m.to].push(crate_buffer[i]);
        }
    }
}

fn move_crates_9001(stacks: &mut [Vec<char>; NUM_STACKS], moves: Vec<CrateMove>) {
    for m in moves {
        let mut crate_buffer = Vec::new();
        for _ in 0..m.num_crates {
            crate_buffer.push(stacks[m.from].pop().unwrap());
        }
        for _ in 0..m.num_crates {
            stacks[m.to].push(crate_buffer.pop().unwrap());
        }
    }
}

pub fn part1(input: String) {
    let mut input = input.split("\n\n");
    let initial_stack_state = input.next().unwrap();
    let mut stacks = read_stacks(&initial_stack_state);
    debug!("{stacks:?}");

    let moves = read_moves(input.next().unwrap().trim());
    debug!("moves: {moves:?}");
    move_crates_9000(&mut stacks, moves);
    let mut ans = "".to_string();
    for i in 0..NUM_STACKS {
        ans = format!("{ans}{}", stacks[i].last().unwrap());
    }
    info!("Part 1 Answer: {ans}");
}

pub fn part2(input: String) {
    let mut input = input.split("\n\n");
    let initial_stack_state = input.next().unwrap();
    let mut stacks = read_stacks(&initial_stack_state);
    debug!("{stacks:?}");

    let moves = read_moves(input.next().unwrap().trim());
    debug!("moves: {moves:?}");
    move_crates_9001(&mut stacks, moves);
    let mut ans = "".to_string();
    for i in 0..NUM_STACKS {
        ans = format!("{ans}{}", stacks[i].last().unwrap());
    }
    info!("Part 2 Answer: {ans}");
}
