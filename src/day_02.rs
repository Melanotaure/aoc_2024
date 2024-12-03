
#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut safe_count = 0;

    for line in lines {
        let report: Vec<&str> = line.split_whitespace().collect();

        let max = report.len();
        let mut current_diff = report[1].parse::<i8>().unwrap() - report[0].parse::<i8>().unwrap();
        if current_diff == 0 || current_diff > 3 {
            continue;
        }

        for i in 1..(max-1) {
            let diff = report[i+1].parse::<i8>().unwrap() - report[i].parse::<i8>().unwrap();
            if diff == 0 {
                break;
            }

        }
    }
    0
}