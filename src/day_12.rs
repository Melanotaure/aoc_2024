use std::collections::HashSet;

#[aoc(day12, part1)]
fn part1(input: &str) -> u32 {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8).collect())
        .collect();

    let mut been_there: HashSet<(i32, i32)> = HashSet::new();

    let mut price = 0;
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if been_there.contains(&(i as i32, j as i32)) {
                continue;
            }
            let res = test_region(&map, i as i32, j as i32, &mut been_there);
            price += res.0 * res.1;
        }
    }
    price
}

fn test_region(
    map: &Vec<Vec<u8>>,
    x: i32,
    y: i32,
    been_there: &mut HashSet<(i32, i32)>,
) -> (u32, u32) {
    let n = (x, y - 1);
    let e = (x + 1, y);
    let s = (x, y + 1);
    let w = (x - 1, y);

    been_there.insert((x, y));

    let mut peri = 0;
    let mut nb_plt = 1;
    let mut jammed = 0;
    if n.1 < 0 || map[y as usize][x as usize] != map[n.1 as usize][n.0 as usize] {
        peri += 1;
        jammed += 1;
    } else {
        if !been_there.contains(&(n.0, n.1)) {
            let res = test_region(&map, n.0, n.1, been_there);
            nb_plt += res.0;
            peri += res.1;
        } else {
            jammed += 1;
        }
    }
    if e.0 >= map[y as usize].len() as i32
        || map[y as usize][x as usize] != map[e.1 as usize][e.0 as usize]
    {
        peri += 1;
    } else {
        if !been_there.contains(&(e.0, e.1)) {
            let res = test_region(&map, e.0, e.1, been_there);
            nb_plt += res.0;
            peri += res.1;
        } else {
            jammed += 1;
        }
    }
    if s.1 >= map.len() as i32 || map[y as usize][x as usize] != map[s.1 as usize][s.0 as usize] {
        peri += 1;
    } else {
        if !been_there.contains(&(s.0, s.1)) {
            let res = test_region(&map, s.0, s.1, been_there);
            nb_plt += res.0;
            peri += res.1;
        } else {
            jammed += 1;
        }
    }
    if w.0 < 0 || map[y as usize][x as usize] != map[w.1 as usize][w.0 as usize] {
        peri += 1;
    } else {
        if !been_there.contains(&(w.0, w.1)) {
            let res = test_region(&map, w.0, w.1, been_there);
            nb_plt += res.0;
            peri += res.1;
        } else {
            jammed += 1;
        };
    }

    if jammed == 4 {
        return (nb_plt, 0);
    }
    (nb_plt, peri)
}
