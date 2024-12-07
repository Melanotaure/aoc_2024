#[aoc(day6, part1)]
fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut map: Vec<&[u8]> = Vec::new();

    for line in lines {
        map.push(line.as_bytes());
    }

    // let's find the starting point
    let mut start: (usize, usize);
    'out: for (j, line) in map.iter().enumerate() {
        for (i, elm) in line.iter().enumerate() {
            if *elm == b'^' {
                start = (i, j);
                break 'out;
            }
        }
    }

    loop {}
    0
}
