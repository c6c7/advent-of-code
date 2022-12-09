#[derive(Clone)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl From<u8> for Movement {
    fn from(b: u8) -> Self {
        match b {
            b'U' => Movement::Up,
            b'D' => Movement::Down,
            b'L' => Movement::Left,
            b'R' => Movement::Right,
            _ => unreachable!(),
        }
    }
}

impl Movement {
    fn from_command(command: &str) -> Vec<Self> {
        let re = regex::Regex::new(r"(U|D|L|R) (\d+)").unwrap();
        let cap = re.captures(command).unwrap();
        let num_moves = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let direction = cap.get(1).unwrap().as_str().as_bytes()[0].into();
        vec![direction; num_moves]
    }
}

#[derive(Clone, Copy)]
struct Head((i64, i64));

impl Head {
    pub fn new() -> Self {
        Self((0, 0))
    }

    pub fn r#move(self, command: Movement) -> (Head, Head) {
        let old_head = self;
        let new_head = match command {
            Movement::Up => Head((old_head.0 .0, old_head.0 .1 + 1)),
            Movement::Down => Head((old_head.0 .0, old_head.0 .1 - 1)),
            Movement::Left => Head((old_head.0 .0 - 1, old_head.0 .1)),
            Movement::Right => Head((old_head.0 .0 + 1, old_head.0 .1)),
        };
        (old_head, new_head)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, std::hash::Hash)]
struct Tail((i64, i64));

impl Tail {
    pub fn new() -> Self {
        Self((0, 0))
    }

    pub fn follow(self, (old_head, new_head): (Head, Head)) -> Self {
        if self.is_adjacent(&new_head) {
            return self;
        }
        Self(old_head.0)
    }

    fn is_adjacent(&self, h: &Head) -> bool {
        self.0 .0 - 1 <= h.0 .0
            && h.0 .0 <= self.0 .0 + 1
            && self.0 .1 - 1 <= h.0 .1
            && h.0 .1 <= self.0 .1 + 1
    }
}

pub fn part1(input: String) {
    let mut tail_positions = std::collections::HashSet::new();
    let mut head = Head::new();
    let mut tail = Tail::new();
    tail_positions.insert(tail);

    for command in input.trim().split('\n') {
        let movements = Movement::from_command(command);
        for m in movements {
            let (old_head, new_head) = head.r#move(m);
            head = new_head;
            tail = tail.follow((old_head, new_head));
            tail_positions.insert(tail);
        }
    }

    tracing::info!("Part 1 Answer: {}", tail_positions.len());
}

pub fn part2(_input: String) {}
