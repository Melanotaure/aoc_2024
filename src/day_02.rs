#[derive(PartialEq)]
enum Tendance {
    Increasing,
    Decreasing,
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut safe_count: u32 = 0;

'outer: for line in lines {
        let report: Vec<&str> = line.split_whitespace().collect();

        let max = report.len();
        let state: Tendance;
        let diff = report[1].parse::<i8>().unwrap() - report[0].parse::<i8>().unwrap();
        if diff < 0 && diff > -4 {
            state = Tendance::Decreasing;
        } else if diff > 0 && diff < 4 {
            state = Tendance::Increasing;
        } else {
            continue;
        }
        for i in 1..(max - 1) {
            let diff = report[i + 1].parse::<i8>().unwrap() - report[i].parse::<i8>().unwrap();
            let current_state: Tendance;
            if diff < 0 && diff > -4 {
                current_state = Tendance::Decreasing;
            } else if diff > 0 && diff < 4 {
                current_state = Tendance::Increasing;
            } else {
                continue 'outer;
            }
            if current_state != state {
                continue 'outer;
            }
        }
        safe_count += 1;
    }
    safe_count
}
