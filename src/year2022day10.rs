use std::convert::TryInto;

#[derive(Debug)]
struct CpuState {
    cycles_completed: usize,
    signal_strength: i64,
    accumulated_signal_strength: i64,
    register_x: i64,
}

impl CpuState {
    pub fn advance_cycle(&mut self) {
        self.cycles_completed += 1;
        self.signal_strength =
            self.register_x * TryInto::<i64>::try_into(self.cycles_completed).unwrap();
        if self.cycles_completed % 40 == 20 {
            self.accumulated_signal_strength += self.signal_strength;
            tracing::debug!("{self:?}");
        }
    }
}

pub fn part1(input: &str) {
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

pub fn part2(_input: &str) {}
