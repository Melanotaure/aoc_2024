#[aoc(day5, part1)]
fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut rules: [[bool; 100]; 100] = [[false; 100]; 100];
    let mut updates: Vec<Vec<u8>> = Vec::new();

    let mut is_rules_section = true;
    for line in lines {
        if line.len() == 0 {
            is_rules_section = false;
            continue;
        }
        if is_rules_section {
            let mut split = line.split("|");
            let p1 = split.next().unwrap().parse::<usize>().unwrap();
            let p2 = split.next().unwrap().parse::<usize>().unwrap();
            rules[p1][p2] = true;
        } else {
            updates.push(line.split(",").map(|m| m.parse::<u8>().unwrap()).collect());
        }
    }

    let mut accu: u32 = 0;
    'out: for update in updates {
        for idx in 0..(update.len() - 1) {
            for i in (idx + 1)..update.len() {
                if !rules[update[idx] as usize][update[i] as usize] {
                    continue 'out;
                }
            }
        }
        let middle = update.len() / 2;
        accu += update[middle] as u32;
    }
    accu
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut rules: [[bool; 100]; 100] = [[false; 100]; 100];
    let mut updates: Vec<Vec<u8>> = Vec::new();

    let mut is_rules_section = true;
    for line in lines {
        if line.len() == 0 {
            is_rules_section = false;
            continue;
        }
        if is_rules_section {
            let mut split = line.split("|");
            let p1 = split.next().unwrap().parse::<usize>().unwrap();
            let p2 = split.next().unwrap().parse::<usize>().unwrap();
            rules[p1][p2] = true;
        } else {
            updates.push(line.split(",").map(|m| m.parse::<u8>().unwrap()).collect());
        }
    }

    let mut accu: u32 = 0;
    'out1: for update in updates {
        for idx in 0..(update.len() - 1) {
            for i in (idx + 1)..update.len() {
                if !rules[update[idx] as usize][update[i] as usize] {
                    accu += reorder(&update, &rules);
                    continue 'out1;
                }
            }
        }
    }
    accu
}

fn reorder(update: &Vec<u8>, rules: &[[bool; 100]; 100]) -> u32 {
    let mut pages = update.clone();
    'out: loop {
        for idx in 0..(update.len() - 1) {
            for i in (idx + 1)..update.len() {
                if !rules[pages[idx] as usize][pages[i] as usize] {
                    pages.swap(idx, i);
                    continue 'out;
                }
            }
        }
        break;
    }
    let middle = update.len() / 2;
    pages[middle] as u32
}
