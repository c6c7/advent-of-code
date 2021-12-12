pub fn part1(input: String) {
    let score = input.split("\n").fold(0, |score_acc, l| {
        let mut open_chunks = vec![];
        for c in l.chars() {
            match c {
                '(' | '[' | '{' | '<' => open_chunks.push(c),
                ')' => {
                    if let Some('(') = open_chunks.pop() {
                    } else {
                        return score_acc + 3;
                    }
                }
                ']' => {
                    if let Some('[') = open_chunks.pop() {
                    } else {
                        return score_acc + 57;
                    }
                }
                '}' => {
                    if let Some('{') = open_chunks.pop() {
                    } else {
                        return score_acc + 1197;
                    }
                }
                '>' => {
                    if let Some('<') = open_chunks.pop() {
                    } else {
                        return score_acc + 25137;
                    }
                }
                _ => unreachable!(),
            }
        }
        score_acc
    });
    println!("Syntax error score: {}", score);
}
