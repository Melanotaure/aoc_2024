#[aoc(day9, part1)]
fn part1(input: &str) -> u64 {
    let disk_map: Vec<u8> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    // build disk
    let mut disk: Vec<i32> = Vec::new();

    let mut is_file_blocks = true;
    let mut id = 0;
    for c in disk_map {
        if is_file_blocks {
            for _ in 0..c {
                disk.push(id);
            }
            id += 1;
            is_file_blocks = false;
        } else {
            for _ in 0..c {
                disk.push(-1);
            }
            is_file_blocks = true;
        }
    }

    let mut right: usize = disk.len() - 1;
    let mut left: usize = 0;
    'out: loop {
        loop {
            if disk[left] == -1 {
                disk.swap(left, right);
                break;
            }
            left += 1;
            if left >= right {
                break 'out;
            }
        }
        right -= 1;
    }

    let mut accu = 0;
    for (i, id) in disk.iter().enumerate() {
        if *id != -1 {
            accu += i as u64 * (*id as u64);
        } else {
            break;
        }
    }
    accu
}
