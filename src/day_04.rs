#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut letters = Vec::new();

    for line in lines {
        letters.push(line.as_bytes());
    }

    let mut count = 0;
    for j in 0..letters.len() {
        let line = letters[j];
        for i in 0..line.len() {
            if line[i] == b'X' {
                count += if h_search(&letters, i, j, 1) { 1 } else { 0 };
                count += if h_search(&letters, i, j, -1) { 1 } else { 0 };
                count += if v_search(&letters, i, j, 1) { 1 } else { 0 };
                count += if v_search(&letters, i, j, -1) { 1 } else { 0 };
                count += if d_search(&letters, i, j, 1, -1) {
                    1
                } else {
                    0
                };
                count += if d_search(&letters, i, j, 1, 1) { 1 } else { 0 };
                count += if d_search(&letters, i, j, -1, 1) {
                    1
                } else {
                    0
                };
                count += if d_search(&letters, i, j, -1, -1) {
                    1
                } else {
                    0
                };
            }
        }
    }

    count
}

fn h_search(letters: &Vec<&[u8]>, x: usize, y: usize, d: isize) -> bool {
    let mut dx = (x as isize + d) as usize;
    if dx >= letters[y].len() {
        return false;
    }
    if letters[y][dx] == b'M' {
        dx = (dx as isize + d) as usize;
        if dx >= letters[y].len() {
            return false;
        }
        if letters[y][dx] == b'A' {
            dx = (dx as isize + d) as usize;
            if dx >= letters[y].len() {
                return false;
            }
            if letters[y][dx] == b'S' {
                return true;
            }
        }
    }
    false
}

fn v_search(letters: &Vec<&[u8]>, x: usize, y: usize, d: isize) -> bool {
    let mut dy = (y as isize + d) as usize;
    if dy >= letters.len() {
        return false;
    }
    if letters[dy][x] == b'M' {
        dy = (dy as isize + d) as usize;
        if dy >= letters.len() {
            return false;
        }
        if letters[dy][x] == b'A' {
            dy = (dy as isize + d) as usize;
            if dy >= letters.len() {
                return false;
            }
            if letters[dy][x] == b'S' {
                return true;
            }
        }
    }
    false
}

fn d_search(letters: &Vec<&[u8]>, x: usize, y: usize, d_x: isize, d_y: isize) -> bool {
    let mut dx = (x as isize + d_x) as usize;
    let mut dy = (y as isize + d_y) as usize;

    if dy >= letters.len() || dx >= letters[dy].len() {
        return false;
    }
    if letters[dy][dx] == b'M' {
        dx = (dx as isize + d_x) as usize;
        dy = (dy as isize + d_y) as usize;
        if dy >= letters.len() || dx >= letters[dy].len() {
            return false;
        }
        if letters[dy][dx] == b'A' {
            dx = (dx as isize + d_x) as usize;
            dy = (dy as isize + d_y) as usize;
            if dy >= letters.len() || dx >= letters[dy].len() {
                return false;
            }
            if letters[dy][dx] == b'S' {
                return true;
            }
        }
    }
    false
}
