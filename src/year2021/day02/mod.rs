fn parse_command<'a>(command: &'a str) -> (&'a str, i64) {
    let mut c = command.split_whitespace();
    let c_name = c
        .next()
        .unwrap_or_else(|| panic!("Command missing first part: {}", command));
    let c_value = c
        .next()
        .unwrap_or_else(|| panic!("Command missing second part: {}", command))
        .parse::<i64>()
        .expect("Failed to parse command value.");
    c.next()
        .map(|_| panic!("Command has unexpected third part: {}", command));
    (c_name, c_value)
}

pub fn part1(input: String) {
    let (final_horizontal, final_depth) =
        input
            .trim()
            .split("\n")
            .fold((0, 0), |(horizontal, depth), command| {
                let (c_name, c_value) = parse_command(command);
                // Increment the horizontal and depth values according
                // to the command.
                match c_name {
                    "forward" => (horizontal + c_value, depth),
                    "up" => (horizontal, depth - c_value),
                    "down" => (horizontal, depth + c_value),
                    _ => panic!("Unexpected command type: {}", c_name),
                }
            });

    println!("Final Horizontal: {}", final_horizontal);
    println!("Final Depth: {}", final_depth);
    println!("Puzzle Answer: {}", final_horizontal * final_depth);
}

pub fn part2(input: String) {
    let (final_horizontal, final_depth, final_aim) =
        input
            .trim()
            .split("\n")
            .fold((0, 0, 0), |(horizontal, depth, aim), command| {
                let (c_name, c_value) = parse_command(command);
                // Increment the horizontal and depth values according
                // to the command.
                match c_name {
                    "forward" => (horizontal + c_value, depth + c_value * aim, aim),
                    "up" => (horizontal, depth, aim - c_value),
                    "down" => (horizontal, depth, aim + c_value),
                    _ => panic!("Unexpected command type: {}", c_name),
                }
            });

    println!("Final Horizontal: {}", final_horizontal);
    println!("Final Depth: {}", final_depth);
    println!("Final Aim: {}", final_aim);
    println!("Puzzle Answer: {}", final_horizontal * final_depth);
}
