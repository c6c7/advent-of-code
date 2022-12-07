use std::{convert::TryInto, str::Split};

fn read_match(input_by_newline: &mut Split<char>) -> Option<(u8, u8)> {
    let raw_match = match input_by_newline.next() {
        Some(raw_match) => raw_match,
        None => return None,
    };
    let mut raw_match_split = raw_match.split_whitespace();
    Some((
        TryInto::<u8>::try_into(raw_match_split.next().unwrap().parse::<char>().unwrap()).unwrap()
            - TryInto::<u8>::try_into('A').unwrap(),
        TryInto::<u8>::try_into(raw_match_split.next().unwrap().parse::<char>().unwrap()).unwrap()
            - TryInto::<u8>::try_into('X').unwrap(),
    ))
}

fn match_score(opponent: u8, me: u8) -> u64 {
    let choice_score = me + 1;
    let outcome_score = if opponent == me {
        3
    } else if (opponent + 1) % 3 == me {
        6
    } else if (me + 1) % 3 == opponent {
        0
    } else {
        unreachable!()
    };
    (choice_score + outcome_score).into()
}

pub fn part1(input: String) {
    let mut input_by_newline = input.trim().split('\n');
    let mut total_score = 0;
    while let Some((opponent, me)) = read_match(&mut input_by_newline) {
        total_score += match_score(opponent, me);
    }
    println!("Part 1 Answer: {total_score}");
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn read_message(input_by_newline: &mut Split<char>) -> Option<(u8, Outcome)> {
    let raw_match = match input_by_newline.next() {
        Some(raw_match) => raw_match,
        None => return None,
    };
    let mut raw_match_split = raw_match.split_whitespace();
    Some((
        TryInto::<u8>::try_into(raw_match_split.next().unwrap().parse::<char>().unwrap()).unwrap()
            - TryInto::<u8>::try_into('A').unwrap(),
        match raw_match_split.next().unwrap().parse::<char>().unwrap() {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        },
    ))
}

fn shape_for_outcome(opponent: u8, outcome: Outcome) -> u8 {
    match outcome {
        Outcome::Win => (opponent + 1) % 3,
        Outcome::Draw => opponent,
        Outcome::Lose => (opponent + 2) % 3,
    }
}

pub fn part2(input: String) {
    let mut input_by_newline = input.trim().split('\n');
    let mut total_score = 0;
    while let Some((opponent, outcome)) = read_message(&mut input_by_newline) {
        total_score += match_score(opponent, shape_for_outcome(opponent, outcome));
    }
    println!("Part 2 Answer: {total_score}");
}
