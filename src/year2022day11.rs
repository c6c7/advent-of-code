use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
struct Throw(usize);

struct Monkey {
    pub number: usize,
    pub items: VecDeque<usize>,
    pub operation: Box<dyn Fn(usize) -> usize>,
    pub test: Box<dyn Fn(usize) -> Option<Throw>>,
    pub test_divisor: usize,
    pub inspections: usize,
}

enum OpArg {
    Old,
    Number(usize),
}

fn parse_op_arg(op_arg: &str) -> OpArg {
    if op_arg == "old" {
        return OpArg::Old;
    }
    OpArg::Number(op_arg.parse::<usize>().unwrap())
}

enum Op {
    Multiply,
    Add,
}

fn parse_op(op: char) -> Op {
    match op {
        '*' => Op::Multiply,
        '+' => Op::Add,
        _ => unreachable!(),
    }
}

fn parse_operation(operation_s: &str) -> Box<dyn Fn(usize) -> usize> {
    let operation_re = regex::Regex::new(r"new = (old|\d+) (\*|\+) (old|\d+)").unwrap();

    let (arg_1, op, arg_2) = {
        let caps = operation_re.captures(operation_s).unwrap();
        (
            parse_op_arg(caps.get(1).unwrap().as_str()),
            parse_op(caps.get(2).unwrap().as_str().chars().next().unwrap()),
            parse_op_arg(caps.get(3).unwrap().as_str()),
        )
    };

    Box::new(move |old| {
        let arg_1 = match arg_1 {
            OpArg::Old => old,
            OpArg::Number(n) => n,
        };
        let arg_2 = match arg_2 {
            OpArg::Old => old,
            OpArg::Number(n) => n,
        };
        match op {
            Op::Multiply => arg_1 * arg_2,
            Op::Add => arg_1 + arg_2,
        }
    })
}

fn parse_test(
    test_s: &str,
    true_monkey_number: usize,
    false_monkey_number: usize,
) -> (Box<dyn Fn(usize) -> Option<Throw>>, usize) {
    let test_re = regex::Regex::new(r"divisible by (\d+)").unwrap();
    let divisor = test_re
        .captures(test_s)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    (
        Box::new(move |current| {
            if current % divisor == 0 {
                Some(Throw(true_monkey_number))
            } else {
                Some(Throw(false_monkey_number))
            }
        }),
        divisor,
    )
}

fn parse_monkey(monkey_definition: &str) -> Monkey {
    let monkey_re = regex::Regex::new(
        r#"
Monkey (?P<monkey_number>\d+):
  Starting items: (?P<items>\d+(, \d+)*)
  Operation: (?P<operation>[^\n]*)
  Test: (?P<test>[^\n]*)
    If true: throw to monkey (?P<true_monkey_number>\d+)
    If false: throw to monkey (?P<false_monkey_number>\d+)"#
            .trim(),
    )
    .unwrap();

    let caps = monkey_re.captures(monkey_definition).unwrap();
    let number = caps
        .name("monkey_number")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let items: VecDeque<usize> = caps
        .name("items")
        .unwrap()
        .as_str()
        .split(", ")
        .map(|item| item.parse::<usize>().unwrap())
        .collect();
    let operation_s = caps.name("operation").unwrap().as_str();
    let test_s = caps.name("test").unwrap().as_str();
    let true_monkey_number = caps
        .name("true_monkey_number")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let false_monkey_number = caps
        .name("false_monkey_number")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    let operation = parse_operation(operation_s);
    let (test, test_divisor) = parse_test(test_s, true_monkey_number, false_monkey_number);

    Monkey {
        number,
        items,
        operation,
        test,
        test_divisor,
        inspections: 0,
    }
}

pub fn part1(input: &str) {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(parse_monkey).collect();
    monkeys.sort_by(|a, b| a.number.cmp(&b.number));
    for round in 1..=20 {
        for i in 0..monkeys.len() {
            let items: Vec<usize> = monkeys[i].items.drain(..).collect();
            for item in items {
                let item = (monkeys[i].operation)(item) / 3;
                monkeys[i].inspections += 1;
                match (monkeys[i].test)(item) {
                    None => unreachable!(),
                    Some(Throw(monkey_number)) => monkeys[monkey_number].items.push_back(item),
                }
            }
        }
        tracing::debug!("After Round {}", round);
        for m in &monkeys {
            tracing::debug!("  Monkey {}: {:?}", m.number, m.items);
        }
    }

    monkeys.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    tracing::info!(
        "Part 1 Answer: {}",
        monkeys
            .iter()
            .rev()
            .take(2)
            .fold(1, |acc, m| acc * m.inspections)
    );
}

pub fn part2(input: &str) {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(parse_monkey).collect();
    monkeys.sort_by(|a, b| a.number.cmp(&b.number));

    let worry_divider = monkeys.iter().fold(1, |acc, m| m.test_divisor * acc);
    for round in 1..=10000 {
        for i in 0..monkeys.len() {
            let items: Vec<usize> = monkeys[i].items.drain(..).collect();
            for item in items {
                let item = (monkeys[i].operation)(item) % worry_divider;
                monkeys[i].inspections += 1;
                match (monkeys[i].test)(item) {
                    None => unreachable!(),
                    Some(Throw(monkey_number)) => monkeys[monkey_number].items.push_back(item),
                }
            }
        }
        tracing::debug!("After Round {}", round);
        for m in &monkeys {
            tracing::debug!("  Monkey {}: {:?}", m.number, m.items);
        }
    }

    monkeys.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    tracing::info!(
        "Part 2 Answer: {}",
        monkeys
            .iter()
            .rev()
            .take(2)
            .fold(1, |acc, m| acc * m.inspections)
    );
}

#[cfg(test)]
mod tests {
    use {super::*, test_case::test_case, tracing_test::traced_test};

    #[traced_test]
    #[test]
    fn parse_monkey_example() {
        let m = parse_monkey(
            r#"
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0"#
                .trim(),
        );
        assert_eq!((m.operation)(3), 9);
        assert_eq!((m.test)(38), Some(Throw(2)));
        assert_eq!((m.test)(39), Some(Throw(0)));
    }

    #[traced_test]
    #[test_case("new = old * old"; "old squared")]
    #[test_case("new = old + old"; "old times two")]
    #[test_case("new = 10 * old"; "ten times old")]
    #[test_case("new = old * 5"; "old times 5")]
    #[test_case("new = old + 12"; "old plus twelve")]
    #[test_case("new = 8 + old"; "eight plus old")]
    fn parse_operation_example(operation_s: &str) {
        let _ = parse_operation(operation_s);
    }
}
