use std::env;

macro_rules! match_day {
    ($arg:expr, $($day:literal => $func:path),+) => {
        match $arg {
            $($day => {
                let (answer1, answer2) = $func(include_str!(concat!("../day", $day, "/input.txt")))?;
                println!("#1: {}", answer1);
                println!("#2: {}", answer2);
            })+
            _ => unimplemented!(),
        }
    };
}

fn main() -> anyhow::Result<()> {
    let day = env::args()
        .nth(1)
        .ok_or(anyhow::anyhow!("Day number is required"))
        .and_then(|s| s.trim().parse().map_err(|e| anyhow::anyhow!("{}", e)))?;
    match_day!(day,
        1 => advent2024::day1::main,
        2 => advent2024::day2::main,
        3 => advent2024::day3::main,
        4 => advent2024::day4::main
    );
    Ok(())
}
