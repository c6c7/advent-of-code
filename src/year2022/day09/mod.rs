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
struct Head(i64, i64);

impl From<Head> for (i64, i64) {
    fn from(h: Head) -> (i64, i64) {
        (h.0, h.1)
    }
}

impl From<Head> for Tail {
    fn from(h: Head) -> Self {
        Self(h.0, h.1)
    }
}

impl Head {
    pub fn new() -> Self {
        Self(0, 0)
    }

    pub fn r#move(self, command: Movement) -> (Head, Head) {
        let old_head = self;
        let new_head = match command {
            Movement::Up => Head(old_head.0, old_head.1 + 1),
            Movement::Down => Head(old_head.0, old_head.1 - 1),
            Movement::Left => Head(old_head.0 - 1, old_head.1),
            Movement::Right => Head(old_head.0 + 1, old_head.1),
        };
        (old_head, new_head)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, std::hash::Hash)]
struct Tail(i64, i64);

impl From<Tail> for (i64, i64) {
    fn from(t: Tail) -> (i64, i64) {
        (t.0, t.1)
    }
}

impl Tail {
    pub fn new() -> Self {
        Self(0, 0)
    }

    fn follow(self, (old, new): ((i64, i64), (i64, i64))) -> (Self, Self) {
        if self.is_adjacent(&new) {
            return (self, self);
        }
        (self, Self(old.0, old.1))
    }

    fn is_adjacent(&self, other: &(i64, i64)) -> bool {
        self.0 - 1 <= other.0
            && other.0 <= self.0 + 1
            && self.1 - 1 <= other.1
            && other.1 <= self.1 + 1
    }
}

struct Rope {
    head: Head,
    rest: [Tail; 9],
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            head: Head::new(),
            rest: [Tail::new(); 9],
        }
    }

    pub fn r#move(mut self, m: Movement) -> Self {
        let (old_head, new_head) = self.head.r#move(m);
        self.head = new_head;
        let (mut old_tail, mut new_tail): (Tail, Tail) = (old_head.into(), new_head.into());
        for t in self.rest.iter_mut() {
            (old_tail, new_tail) = t.follow((old_tail.into(), new_tail.into()));
            *t = new_tail;
        }
        self
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
            tail = tail.follow((old_head.into(), new_head.into())).1;
            tail_positions.insert(tail);
        }
    }

    tracing::info!("Part 1 Answer: {}", tail_positions.len());
}

pub fn part2(input: String) {
    let mut rope = Rope::new();
    let mut tail_positions = std::collections::HashSet::new();
    tail_positions.insert(rope.rest[8]);

    for command in input.trim().split('\n') {
        let movements = Movement::from_command(command);
        for m in movements {
            rope = rope.r#move(m);
            tail_positions.insert(rope.rest[8]);
        }
    }

    tracing::info!("Part 2 Answer: {}", tail_positions.len());
}
