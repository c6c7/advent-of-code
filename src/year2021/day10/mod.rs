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
