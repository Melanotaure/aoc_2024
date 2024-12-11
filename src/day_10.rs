#[aoc(day10, part1)]
fn part1(input: &str) -> u32 {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut accu = 0;
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            accu += if map[j][i] == 0 {
                let mut summits: Vec<(i32, i32)> = Vec::new();
                follow_path(&map, i as i32, j as i32, &mut summits);
                summits.len()
            } else {
                0
            };
        }
    }
    accu as u32
}

fn follow_path(map: &Vec<Vec<u8>>, x: i32, y: i32, mut summits: &mut Vec<(i32, i32)>) {
    let n = (x, y - 1);
    let e = (x + 1, y);
    let s = (x, y + 1);
    let o = (x - 1, y);

    if n.1 >= 0 && map[n.1 as usize][n.0 as usize] == map[y as usize][x as usize] + 1 {
        if map[n.1 as usize][n.0 as usize] == 9 {
            if !summits.contains(&n) {
                summits.push(n);
            }
        } else {
            follow_path(&map, n.0, n.1, &mut summits);
        }
    }

    if (e.0 as usize) < map[y as usize].len()
        && map[e.1 as usize][e.0 as usize] == map[y as usize][x as usize] + 1
    {
        if map[e.1 as usize][e.0 as usize] == 9 {
            if !summits.contains(&e) {
                summits.push(e);
            }
        } else {
            follow_path(&map, e.0, e.1, &mut summits);
        }
    }

    if (s.1 as usize) < map.len()
        && map[s.1 as usize][s.0 as usize] == map[y as usize][x as usize] + 1
    {
        if map[s.1 as usize][s.0 as usize] == 9 {
            if !summits.contains(&s) {
                summits.push(s);
            }
        } else {
            follow_path(&map, s.0, s.1, &mut summits);
        }
    }

    if o.0 >= 0 && map[o.1 as usize][o.0 as usize] == map[y as usize][x as usize] + 1 {
        if map[o.1 as usize][o.0 as usize] == 9 {
            if !summits.contains(&o) {
                summits.push(o);
            }
        } else {
            follow_path(&map, o.0, o.1, &mut summits);
        }
    }
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u32 {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut accu = 0;
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            accu += if map[j][i] == 0 {
                let mut summits: Vec<(i32, i32)> = Vec::new();
                follow_all_path(&map, i as i32, j as i32, &mut summits);
                summits.len()
            } else {
                0
            };
        }
    }
    accu as u32
}

fn follow_all_path(map: &Vec<Vec<u8>>, x: i32, y: i32, mut summits: &mut Vec<(i32, i32)>) {
    let n = (x, y - 1);
    let e = (x + 1, y);
    let s = (x, y + 1);
    let o = (x - 1, y);

    if n.1 >= 0 && map[n.1 as usize][n.0 as usize] == map[y as usize][x as usize] + 1 {
        if map[n.1 as usize][n.0 as usize] == 9 {
            summits.push(n);
        } else {
            follow_all_path(&map, n.0, n.1, &mut summits);
        }
    }

    if (e.0 as usize) < map[y as usize].len()
        && map[e.1 as usize][e.0 as usize] == map[y as usize][x as usize] + 1
    {
        if map[e.1 as usize][e.0 as usize] == 9 {
            summits.push(e);
        } else {
            follow_all_path(&map, e.0, e.1, &mut summits);
        }
    }

    if (s.1 as usize) < map.len()
        && map[s.1 as usize][s.0 as usize] == map[y as usize][x as usize] + 1
    {
        if map[s.1 as usize][s.0 as usize] == 9 {
            summits.push(s);
        } else {
            follow_all_path(&map, s.0, s.1, &mut summits);
        }
    }

    if o.0 >= 0 && map[o.1 as usize][o.0 as usize] == map[y as usize][x as usize] + 1 {
        if map[o.1 as usize][o.0 as usize] == 9 {
            summits.push(o);
        } else {
            follow_all_path(&map, o.0, o.1, &mut summits);
        }
    }
}
