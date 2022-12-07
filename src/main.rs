use {
    advent_of_code::get_day,
    std::{
        env, fs, io,
        time::{Duration, Instant},
    },
    tracing::{debug, info},
};

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return format!("{}µs", micro_sec.round());
    }
    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return format!("{whole_ms}ms ") + &fmt_time(rem_ms);
    }
    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;
        return format!("{whole_sec}s ") + &fmt_time(rem_ms);
    }
    let min: f64 = sec / 60.0;
    format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0)
}

fn fmt_dur(dur: Duration) -> String {
    fmt_time(dur.as_secs_f64() * 1000.0)
}

fn main() {
    tracing_subscriber::fmt::init();

    // Get day string
    let args: Vec<String> = env::args().collect();
    let mut year = String::new();
    let mut day = String::new();

    if args.len() >= 2 {
        year = args[1].clone();
        day = args[2].clone();
    } else {
        info!("Enter year: ");
        io::stdin()
            .read_line(&mut year)
            .expect("Failed to read line");
        info!("Enter day: ");
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line");
    }
    // Parse year as number
    year = year.trim().to_string();
    let year_num: u32 = match year.parse() {
        Ok(num) => num,
        Err(_) => {
            info!("Invalid year number: {}", year);
            return;
        }
    };
    // Parse day as number
    day = day.trim().to_string();
    let day_num: u32 = match day.parse() {
        Ok(num) => num,
        Err(_) => {
            info!("Invalid day number: {}", day);
            return;
        }
    };
    // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd
        .join("inputs")
        .join(format!("year{year_num:04}/day{day_num:02}.txt"));
    info!("Reading {}", filename.display());
    let input = fs::read_to_string(filename).expect("Error while reading");

    // Get corresponding function

    let (part1, part2) = get_day(year_num, day_num);
    // Time it
    info!("Running Part 1");
    let part1_start = Instant::now();
    part1(input.clone());
    let part1_dur = part1_start.elapsed();
    debug!("Took {}", fmt_dur(part1_dur));

    info!("Running Part 2");
    let part2_start = Instant::now();
    part2(input);
    let part2_dur = part2_start.elapsed();
    debug!("Took {}", fmt_dur(part2_dur));
}
