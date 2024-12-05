use regex::Regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let split: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    let mut accu = 0;

    for txt in split {
        let mut tmp1 = txt.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().split(",");
        let val1 = tmp1.next().unwrap().parse::<u32>().unwrap();
        let val2 = tmp1.next().unwrap().parse::<u32>().unwrap();
        accu += val1 * val2;
    }

    accu
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|don't\(\)|do\(\)").unwrap();
    let split: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
    let mut accu = 0;

    let mut dos: Vec<usize> = Vec::new();
    let mut store= true;
    for (i, txt) in split.iter().enumerate() {
        if *txt == "don't()" {
            store = false;
        } else if *txt == "do()" {
            store = true;
            continue;
        }
        if store {
            dos.push(i);
        }
    }

    for idx in dos {
        let mut tmp1 = split[idx].strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().split(",");
        let val1 = tmp1.next().unwrap().parse::<u32>().unwrap();
        let val2 = tmp1.next().unwrap().parse::<u32>().unwrap();
        accu += val1 * val2;
    }

    accu
}
