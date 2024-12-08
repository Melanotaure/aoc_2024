struct Antenna {
    freq: u8,
    x: i32,
    y: i32,
}

#[aoc(day8, part1)]
fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let antenna_map: Vec<&[u8]> = lines.map(|m| m.as_bytes()).collect();
    let mut antinodes: Vec<Vec<u8>> = Vec::new();

    for row in &antenna_map {
        antinodes.push(vec![b'.'; row.len()]);
    }

    'out: for j in 0..antenna_map.len() {
        for i in 0..antenna_map[j].len() {
            let current_antenna = if antenna_map[j][i] != b'.' {
                Antenna {
                    freq: antenna_map[j][i],
                    x: i as i32,
                    y: j as i32,
                }
            } else {
                continue;
            };
            let mut jj = j;
            let mut ii = if (i + 1) >= antenna_map[j].len() {
                if (j + 1) >= antenna_map.len() {
                    break 'out;
                } else {
                    jj = j + 1;
                }
                0
            } else {
                i + 1
            };
            loop {
                loop {
                    if antenna_map[jj][ii] == current_antenna.freq {
                        let v = (ii as i32 - current_antenna.x, jj as i32 - current_antenna.y);
                        let n1 = (current_antenna.x - v.0, current_antenna.y - v.1);
                        let n2 = (current_antenna.x + 2 * v.0, current_antenna.y + 2 * v.1);
                        place_nodes(&mut antinodes, n1, n2);
                    }
                    ii += 1;
                    if ii >= antenna_map[jj].len() {
                        ii = 0;
                        break;
                    }
                }
                jj += 1;
                if jj >= antenna_map.len() {
                    break;
                }
            }
        }
    }

    let mut count = 0;
    for row in antinodes {
        for c in row {
            if c == b'#' {
                count += 1;
            }
        }
    }
    count
}

fn place_nodes(node_map: &mut Vec<Vec<u8>>, n1: (i32, i32), n2: (i32, i32)) {
    if n1.1 >= 0
        && n1.1 < node_map.len() as i32
        && n1.0 >= 0
        && n1.0 < node_map[n1.1 as usize].len() as i32
    {
        node_map[n1.1 as usize][n1.0 as usize] = b'#';
    }

    if n2.1 >= 0
        && n2.1 < node_map.len() as i32
        && n2.0 >= 0
        && n2.0 < node_map[n2.1 as usize].len() as i32
    {
        node_map[n2.1 as usize][n2.0 as usize] = b'#';
    }
}

#[aoc(day8, part2)]
fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let antenna_map: Vec<&[u8]> = lines.map(|m| m.as_bytes()).collect();
    let mut antinodes: Vec<Vec<u8>> = Vec::new();

    for row in &antenna_map {
        antinodes.push(vec![b'.'; row.len()]);
    }

    'out: for j in 0..antenna_map.len() {
        for i in 0..antenna_map[j].len() {
            let current_antenna = if antenna_map[j][i] != b'.' {
                Antenna {
                    freq: antenna_map[j][i],
                    x: i as i32,
                    y: j as i32,
                }
            } else {
                continue;
            };
            let mut jj = j;
            let mut ii = if (i + 1) >= antenna_map[j].len() {
                if (j + 1) >= antenna_map.len() {
                    break 'out;
                } else {
                    jj = j + 1;
                }
                0
            } else {
                i + 1
            };
            loop {
                loop {
                    if antenna_map[jj][ii] == current_antenna.freq {
                        let v = (ii as i32 - current_antenna.x, jj as i32 - current_antenna.y);
                        place_res_nodes(&mut antinodes, (current_antenna.x, current_antenna.y), v);
                    }
                    ii += 1;
                    if ii >= antenna_map[jj].len() {
                        ii = 0;
                        break;
                    }
                }
                jj += 1;
                if jj >= antenna_map.len() {
                    break;
                }
            }
        }
    }

    let mut count = 0;
    for row in antinodes {
        for c in row {
            if c == b'#' {
                count += 1;
            }
        }
    }
    count
}

fn place_res_nodes(node_map: &mut Vec<Vec<u8>>, n0: (i32, i32), v: (i32, i32)) {
    let mut n = n0;

    while n.1 >= 0
        && n.1 < node_map.len() as i32
        && n.0 >= 0
        && n.0 < node_map[n.1 as usize].len() as i32
    {
        node_map[n.1 as usize][n.0 as usize] = b'#';
        n = (n.0 - v.0, n.1 - v.1);
    }

    n = (n0.0 + v.0, n0.1 + v.1);
    while n.1 >= 0
        && n.1 < node_map.len() as i32
        && n.0 >= 0
        && n.0 < node_map[n.1 as usize].len() as i32
    {
        node_map[n.1 as usize][n.0 as usize] = b'#';
        n = (n.0 + v.0, n.1 + v.1);
    }
}
