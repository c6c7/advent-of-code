use {
    std::{env, fs, io},
    tracing::info,
};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info,[main]=debug")
        .init();

    advent_of_code::init_parts();

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
    let year_num: usize = match year.parse() {
        Ok(num) => num,
        Err(_) => {
            info!("Invalid year number: {}", year);
            return;
        }
    };
    // Parse day as number
    day = day.trim().to_string();
    let day_num: u8 = match day.parse() {
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

    let parts = advent_of_code::PARTS.get(&(year_num, day_num));
    for (i, part) in parts.iter().enumerate().map(|(i, part)| (i + 1, part)) {
        info!("Running Part {}", i);
        part(&input);
        info!("Part {} complete.", i);
    }
}
