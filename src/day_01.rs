#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut list1 = Vec::<u32>::new();
    let mut list2 = Vec::<u32>::new();

    for line in lines {
        let mut split_line = line.split_whitespace();
        list1.push(split_line.nth(0).unwrap().parse::<u32>().unwrap());
        list2.push(split_line.nth(0).unwrap().parse::<u32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut distance = 0;
    for (index, v1) in list1.iter().enumerate() {
        let v2 = list2[index];
        distance += (*v1 as i32 - v2 as i32).abs() as u32;
    }

    distance
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in lines {
        let mut split_line = line.split_whitespace();
        list1.push(split_line.nth(0).unwrap().parse::<u32>().unwrap());
        list2.push(split_line.nth(0).unwrap().parse::<u32>().unwrap());
    }

    let mut similarity = 0;
    for v1 in list1 {
        let mut count = 0;
        for v2 in &list2 {
            if v1 == *v2 {
                count += 1;
            }
        }
        similarity += v1 * count;
    }
    similarity
}