#[derive(Debug)]
struct CpuState {
    cycles_completed: i64,
    signal_strength: i64,
    accumulated_signal_strength: i64,
    register_x: i64,
}

impl CpuState {
    pub fn advance_cycle(&mut self) {
        self.cycles_completed += 1;
        self.signal_strength = self.register_x * self.cycles_completed;
        if self.cycles_completed % 40 == 20 {
            self.accumulated_signal_strength += self.signal_strength;
            tracing::debug!("{self:?}");
        }
    }
}

struct Sprite(i64);

struct Crt {
    display: [[char; 40]; 6],
    cpu: CpuState,
    sprite: Sprite,
}

impl Crt {
    pub fn render(&mut self) {
        let pixel = &mut self.display[(self.cpu.cycles_completed / 40) as usize]
            [(self.cpu.cycles_completed % 40) as usize];
        let cursor = self.cpu.cycles_completed % 40;
        if self.sprite.0 - 1 <= cursor && cursor <= self.sprite.0 + 1 {
            *pixel = '#';
        } else {
            *pixel = '.';
        }
    }

    pub fn noop(&mut self) {
        self.render();
        self.cpu.advance_cycle();
    }
    pub fn addx(&mut self, inc: i64) {
        self.render();
        self.cpu.advance_cycle();
        self.render();
        self.cpu.advance_cycle();
        self.cpu.register_x += inc;
        self.sprite.0 = self.cpu.register_x;
    }
}

impl std::fmt::Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, " {}", ['-'; 39].iter().collect::<String>())?;
        for row in self.display {
            writeln!(f, "|{}|", row.iter().collect::<String>())?;
        }
        writeln!(f, " {}", ['-'; 39].iter().collect::<String>())
    }
}

pub fn part1(input: String) {
    let cpu_state = input
        .trim()
        .split('\n')
        .try_fold(
            CpuState {
                cycles_completed: 0,
                signal_strength: 0,
                accumulated_signal_strength: 0,
                register_x: 1,
            },
            |mut cpu_state, line| {
                match &line.as_bytes()[..4] {
                    b"noop" => {
                        cpu_state.advance_cycle();
                    }
                    b"addx" => {
                        cpu_state.advance_cycle();
                        cpu_state.advance_cycle();
                        cpu_state.register_x += String::from_utf8(line.as_bytes()[5..].to_vec())
                            .unwrap()
                            .parse::<i64>()
                            .unwrap();
                    }
                    _ => unreachable!(),
                }
                Ok::<CpuState, ()>(cpu_state)
            },
        )
        .unwrap();

    tracing::debug!("{cpu_state:?}");
    tracing::info!("Part 1 Answer: {}", cpu_state.accumulated_signal_strength);
}

pub fn part2(input: String) {
    let crt = input
        .trim()
        .split('\n')
        .try_fold(
            Crt {
                cpu: CpuState {
                    cycles_completed: 0,
                    signal_strength: 0,
                    accumulated_signal_strength: 0,
                    register_x: 1,
                },
                display: [['-'; 40]; 6],
                sprite: Sprite(1),
            },
            |mut crt, line| {
                match &line.as_bytes()[..4] {
                    b"noop" => {
                        crt.noop();
                    }
                    b"addx" => {
                        crt.addx(
                            String::from_utf8(line.as_bytes()[5..].to_vec())
                                .unwrap()
                                .parse::<i64>()
                                .unwrap(),
                        );
                    }
                    _ => unreachable!(),
                }
                Ok::<Crt, ()>(crt)
            },
        )
        .unwrap();

    tracing::info!("Part 2 Answer:\n{}", crt);
}
