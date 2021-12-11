pub fn part1(input: String) {
    let unique_signal_patterns = input.split("\n").fold(0, |acc, l| {
        let mut parts = l.split("|");
        let _patterns = parts.next().unwrap().trim();
        let output_value = parts.next().unwrap().trim();
        acc + output_value.split_whitespace().fold(0, |acc, p| {
            if let Some(_) = [2, 4, 3, 7].iter().find(|&&x| p.len() == x) {
                acc + 1
            } else {
                acc
            }
        })
    });
    println!("Unique signal patterns: {}", unique_signal_patterns);
}
