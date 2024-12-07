enum Obstacle {
    Nothing,
    Object,
    Visited,
    Exit,
}

#[derive(Debug)]
struct Bot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Bot {
    fn new(x: i32, y: i32, dx: i32, dy: i32) -> Self {
        Self { x, y, dx, dy }
    }

    fn move_forward(&mut self) {
        self.x += self.dx;
        self.y -= self.dy;
    }

    fn turn_right(&mut self) {
        let tmp = self.dx;
        self.dx = self.dy;
        self.dy = -tmp;
    }
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in lines {
        let mut l: Vec<u8> = Vec::new();
        line.as_bytes().clone_into(&mut l);
        map.push(l);
    }

    // let's find the starting point
    let mut start = (0, 0);
    'out: for (j, line) in map.iter().enumerate() {
        for (i, elm) in line.iter().enumerate() {
            if *elm == b'^' {
                start = (i, j);
                break 'out;
            }
        }
    }

    let mut guard = Bot::new(start.0 as i32, start.1 as i32, 0, 1);
    let mut move_count = 1;
    loop {
        match test_forward(&map, &guard) {
            Obstacle::Exit => break,
            Obstacle::Nothing => {
                map[guard.y as usize][guard.x as usize] = b'X';
                guard.move_forward();
                move_count += 1;
            }
            Obstacle::Visited => {
                map[guard.y as usize][guard.x as usize] = b'X';
                guard.move_forward();
            }
            Obstacle::Object => guard.turn_right(),
        }
    }

    move_count
}

fn test_forward(map: &Vec<Vec<u8>>, bot: &Bot) -> Obstacle {
    let fx = bot.x + bot.dx;
    let fy = bot.y - bot.dy;
    if fy < 0 || fx < 0 || fy >= map.len() as i32 || fx >= map[fy as usize].len() as i32 {
        return Obstacle::Exit;
    }
    if map[fy as usize][fx as usize] == b'.' {
        return Obstacle::Nothing;
    }
    if map[fy as usize][fx as usize] == b'#' {
        return Obstacle::Object;
    }
    if map[fy as usize][fx as usize] == b'X' {
        return Obstacle::Visited;
    }
    Obstacle::Exit
}
