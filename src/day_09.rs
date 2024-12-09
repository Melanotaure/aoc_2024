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

#[aoc(day9, part2)]
fn part2(input: &str) -> u64 {
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
    let mut left: usize;
    'out: loop {
        let id = disk[right];
        // count ids
        if id == -1 {
            right -= 1;
            if right == 0 {
                break;
            } else {
                continue;
            }
        }

        let mut id_count = 1;
        loop {
            right -= 1;
            if right == 0 {
                break 'out;
            }

            if disk[right] != id {
                break;
            }
            id_count += 1
        }

        left = 0;
        let mut free_blocks_count = 0;
        'l_search: loop {
            // is there a free block
            if disk[left] != -1 {
                left += 1;
                if left > right {
                    continue 'out;
                }
                continue;
            } else {
                // Count the free blocks
                free_blocks_count += 1;
                loop {
                    left += 1;
                    if disk[left] == -1 {
                        free_blocks_count += 1
                    } else {
                        // Does it fit in ?
                        if free_blocks_count >= id_count {
                            // Yes, place the file blocks together
                            left -= free_blocks_count;
                            right += id_count;
                            for _ in 0..id_count {
                                disk.swap(left, right);
                                left += 1;
                                right -= 1;
                            }
                            continue 'out;
                        } else {
                            // No, continue the searching
                            free_blocks_count = 0;
                            continue 'l_search;
                        }
                    }
                }
            }
        }
    }

    let mut accu = 0;
    for (i, id) in disk.iter().enumerate() {
        if *id != -1 {
            accu += i as u64 * (*id as u64);
        }
    }
    accu
}
