#[derive(PartialEq)]
enum Tendance {
    Increasing,
    Decreasing,
}

enum Safety {
    Safe,
    Unsafe,
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut safe_count: u32 = 0;

    for line in lines {
        let report: Vec<&str> = line.split_whitespace().collect();
        let safe_state: Safety;

        (safe_state, _, _) = test_safety(&report);
        match safe_state {
            Safety::Safe => safe_count += 1,
            Safety::Unsafe => {}
        }
    }
    safe_count
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut safe_count: u32 = 0;
    let mut count = 0;

    'outer: for (i, line) in lines.into_iter().enumerate() {
        let report: Vec<&str> = line.split_whitespace().collect();
        let safe_state: Safety;
        let rm_id1: usize;
        let rm_id2: usize;

        (safe_state, rm_id1, rm_id2) = test_safety(&report);
        match safe_state {
            Safety::Safe => safe_count += 1,
            Safety::Unsafe => {
                let mut creport = report.clone();
                creport.remove(rm_id1);
                let safe_state: Safety;
                (safe_state, _, _) = test_safety(&creport);
                match safe_state {
                    Safety::Safe => {
                        safe_count += 1;
                        continue 'outer;
                    }
                    Safety::Unsafe => {}
                }
                let mut creport = report.clone();
                creport.remove(rm_id2);
                let safe_state: Safety;
                (safe_state, _, _) = test_safety(&creport);
                match safe_state {
                    Safety::Safe => {
                        safe_count += 1;
                        continue 'outer;
                    }
                    Safety::Unsafe => {}
                }
            }
        }
        count = i;
    }
    println!("{count}");
    safe_count
}

fn test_safety(report: &Vec<&str>) -> (Safety, usize, usize) {
    let state: Tendance;
    let diff = report[1].parse::<i8>().unwrap() - report[0].parse::<i8>().unwrap();
    if diff < 0 && diff > -4 {
        state = Tendance::Decreasing;
    } else if diff > 0 && diff < 4 {
        state = Tendance::Increasing;
    } else {
        return (Safety::Unsafe, 0, 1);
    }
    for i in 1..(report.len() - 1) {
        let diff = report[i + 1].parse::<i8>().unwrap() - report[i].parse::<i8>().unwrap();
        let current_state: Tendance;
        if diff < 0 && diff > -4 {
            current_state = Tendance::Decreasing;
        } else if diff > 0 && diff < 4 {
            current_state = Tendance::Increasing;
        } else {
            return (Safety::Unsafe, i, i + 1);
        }
        if current_state != state {
            return (Safety::Unsafe, i, i + 1);
        }
    }
    (Safety::Safe, 0, 0)
}
