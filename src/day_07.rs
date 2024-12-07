#[aoc(day7, part1)]
fn part1(input: &str) -> u64 {
    let lines = input.lines();
    let mut results: Vec<u64> = Vec::new();
    let mut equations: Vec<Vec<u64>> = Vec::new();

    for line in lines {
        let mut split = line.split(": ");
        let result = split.next().unwrap().parse::<u64>().unwrap();
        let values: Vec<u64> = split
            .next()
            .unwrap()
            .split(" ")
            .map(|m| m.parse::<u64>().unwrap())
            .collect();
        results.push(result);
        equations.push(values);
    }

    let mut accu = 0;
    for (i, res) in results.iter().enumerate() {
        if is_equ_ok(*res, &equations[i]) {
            accu += res;
        }
    }
    accu
}

fn is_equ_ok(result: u64, values: &Vec<u64>) -> bool {
    let mut res: Vec<(u64, u64)> = Vec::new();
    res.push(add_mul(values[0], values[1]));
    if values.len() > 2 {
        for i in 2..values.len() {
            let mut rtmp: Vec<(u64, u64)> = Vec::new();
            for r in &res {
                let r1 = add_mul(r.0, values[i]);
                let r2 = add_mul(r.1, values[i]);
                rtmp.push(r1);
                rtmp.push(r2);
            }
            res.clear();
            res = rtmp.clone();
        }
    }
    for r in res {
        if result == r.0 || result == r.1 {
            return true;
        }
    }
    false
}

fn add_mul(v1: u64, v2: u64) -> (u64, u64) {
    (v1 + v2, v1 * v2)
}

fn add_mul_concat(v1: u64, v2: u64) -> (u64, u64, u64) {
    let mut str1 = v1.to_string();
    let str2 = v2.to_string();
    str1 = str1 + &str2;
    let v3 = str1.parse::<u64>().unwrap();
    (v1 + v2, v1 * v2, v3)
}
