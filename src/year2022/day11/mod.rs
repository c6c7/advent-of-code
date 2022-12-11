#[derive(PartialEq, Eq, Debug)]
struct Throw(u8);

struct Monkey {
    pub number: u8,
    pub items: Vec<u8>,
    pub operation: Box<dyn Fn(u8) -> u8>,
    pub test: Box<dyn Fn(u8) -> Option<Throw>>,
}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.number.eq(&other.number)
    }
}

impl Eq for Monkey {}

enum OpArg {
    Old,
    Number(u8),
}

fn parse_op_arg(op_arg: &str) -> OpArg {
    if op_arg == "old" {
        return OpArg::Old;
    }
    OpArg::Number(op_arg.parse::<u8>().unwrap())
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

fn parse_operation(operation_s: &str) -> Box<dyn Fn(u8) -> u8> {
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
    true_monkey_number: u8,
    false_monkey_number: u8,
) -> Box<dyn Fn(u8) -> Option<Throw>> {
    let test_re = regex::Regex::new(r"divisible by (\d+)").unwrap();
    let divisor = test_re.captures(test_s).unwrap().get(1).unwrap().as_str().parse::<u8>().unwrap();
    Box::new(move |current| if current % divisor == 0 {
        Some(Throw(true_monkey_number))
    } else {
        Some(Throw(false_monkey_number))
    })
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
        .parse::<u8>()
        .unwrap();
    let items: Vec<u8> = caps
        .name("items")
        .unwrap()
        .as_str()
        .split(", ")
        .map(|item| item.parse::<u8>().unwrap())
        .collect();
    let operation_s = caps.name("operation").unwrap().as_str();
    let test_s = caps.name("test").unwrap().as_str();
    let true_monkey_number = caps
        .name("true_monkey_number")
        .unwrap()
        .as_str()
        .parse::<u8>()
        .unwrap();
    let false_monkey_number = caps
        .name("false_monkey_number")
        .unwrap()
        .as_str()
        .parse::<u8>()
        .unwrap();

    let operation = parse_operation(operation_s);
    let test = parse_test(test_s, true_monkey_number, false_monkey_number);

    Monkey {
        number,
        items,
        operation,
        test,
    }
}

pub fn part1(input: String) {
    let monkeys = input.split("\n\n").map(parse_monkey);
    for m in monkeys {
        tracing::debug!("Monkey {}: {:?}", m.number, m.items);
    }
}

pub fn part2(_input: String) {}

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
        parse_operation(operation_s);
    }
}
