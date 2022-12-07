#![allow(clippy::all)]

enum ParseResult {
    Incomplete(Vec<char>),
    SyntaxError(u32),
}

fn parse_chunks(chunks: &str) -> ParseResult {
    let mut open_chunks = vec![];

    macro_rules! syntax_error_check {
        ($open:literal, $score:literal) => {
            if let Some($open) = open_chunks.pop() {
            } else {
                return ParseResult::SyntaxError($score);
            }
        };
    }

    for c in chunks.chars() {
        match c {
            '(' | '[' | '{' | '<' => open_chunks.push(c),
            ')' => syntax_error_check!('(', 3),
            ']' => syntax_error_check!('[', 57),
            '}' => syntax_error_check!('{', 1197),
            '>' => syntax_error_check!('<', 25137),
            _ => unreachable!(),
        }
    }

    ParseResult::Incomplete(open_chunks)
}

pub fn part1(input: String) {
    let score = input.split("\n").fold(0, |score_acc, l| {
        if let ParseResult::SyntaxError(score) = parse_chunks(l) {
            return score_acc + score;
        }
        score_acc
    });
    println!("Syntax error score: {}", score);
}

pub fn part2(input: String) {
    let open_chunk_list = input.split("\n").fold(vec![], |mut open_chunks_acc, l| {
        if let ParseResult::Incomplete(open_chunk) = parse_chunks(l) {
            open_chunks_acc.push(open_chunk);
        }
        open_chunks_acc
    });

    let mut scores = vec![];
    for mut open_chunk in open_chunk_list {
        let mut score = 0u64;
        while !open_chunk.is_empty() {
            score *= 5;
            match open_chunk.pop().unwrap() {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => unreachable!(),
            }
        }
        scores.push(score)
    }
    scores.sort();
    println!("Answer: {}", scores[scores.len() / 2]);
}
