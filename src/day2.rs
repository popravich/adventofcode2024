pub fn main(input: &str) -> anyhow::Result<(usize, usize)> {
    let mut part1 = 0;

    let mut tmp = vec![];
    for line in input.lines() {
        tmp.clear();
        for w in line.split(' ') {
            let level = w.parse::<usize>()?;
            tmp.push(level);
        }

        if let Report::Safe = make_report(&tmp) {
            part1 += 1;
        }
    }

    let mut part2 = 0;
    for line in input.lines() {
        tmp.clear();
        for w in line.split(' ') {
            let level = w.parse::<usize>()?;
            tmp.push(level);
        }

        match make_report(&tmp) {
            Report::Safe => {
                part2 += 1;
            }
            Report::Unsafe => {
                for idx in 0..tmp.len() {
                    let mut xxx = tmp.clone();
                    xxx.remove(idx);
                    if let Report::Safe = make_report(&xxx) {
                        part2 += 1;
                        break;
                    }
                }
            }
        }
    }
    Ok((part1, part2))
}

enum Report {
    Safe,
    Unsafe,
}

#[derive(Clone, Copy)]
enum State {
    Increasing,
    Decreasing,
}

fn make_report(levels: &[usize]) -> Report {
    use self::Report::*;
    use self::State::*;

    let mut last_state: Option<State> = None;
    let stream = levels.windows(2).map(|w| (w[0], w[1]));
    for pair in stream {
        match (last_state.as_ref().copied(), pair) {
            (_, (a, b)) if !level_in_range(a, b) => return Unsafe,
            (Some(Decreasing), (a, b)) if a > b => continue,
            (Some(Increasing), (a, b)) if a < b => continue,
            (None, (a, b)) => {
                if a > b {
                    last_state.replace(Decreasing);
                } else {
                    last_state.replace(Increasing);
                }
            }
            _ => return Unsafe,
        };
    }
    Safe
}

fn level_in_range(a: usize, b: usize) -> bool {
    !(a == b || a.abs_diff(b) > 3)
}
