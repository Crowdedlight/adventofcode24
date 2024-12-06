use std::collections::HashSet;

#[derive(Debug, Default)]
struct Matrix {
    pub rows: Vec<Vec<char>>
}
impl Matrix {
    pub fn get(&self, x: i32, y: i32) -> Option<char> {

        if x < 0 || y < 0 {
            return None;
        }

        Some(*self.rows.get(y as usize)?.get(x as usize)?)
    }
    pub fn get_with_pos(&self, pos: (i32, i32)) -> Option<char> {
        self.get(pos.0, pos.1)
    }
    pub fn add_row(&mut self, row: Vec<char>) {
        self.rows.push(row);
    }

    pub fn new(input: &str) -> Self {
        let mut matrix = Matrix::default();

        for line in input.lines() {
            matrix.add_row(line.chars().collect());
        }
        matrix
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        self.rows[y][x] = c;
    }

    pub fn set_all_to(&mut self, input: char) {
        self.rows.iter_mut().for_each(|i| {
            i.iter_mut().for_each(|c| *c = input);
        });
    }

    pub fn print(&self) {
        for row in self.rows.iter() {
            println!("{:?}", row.iter().collect::<String>());
        }
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

pub fn process(input: &str) -> anyhow::Result<String> {

    // matrix again, use a get function returning Option and input of i32, with < 0, checks.
    // so we can just get the start position and then move until obstacles or outside of map

    //  If there is something directly in front of you, turn right 90 degrees.
    //  Otherwise, take a step forward.

    // load the matrix
    let mut matrix = Matrix::new(input);

    // find stat location
    let mut start_pos: (i32, i32) = (0, 0);

    for (idx, row) in matrix.rows.iter().enumerate() {
        let pos = row.iter().position(|&c| c == '^');

        if pos.is_some() {
            start_pos = (pos.unwrap() as i32, idx as i32);
            break;
        }
    }

    // main vars
    let mut path: HashSet<(i32, i32)> = HashSet::new();
    let mut dir = Direction::Up;
    let mut curr_pos = start_pos;

    // add start pos to hashset
    path.insert(start_pos);

    // main loop, keep going until new pos is None
    while true {
        // clone pos to keep last pos in case new pos is not valid
        let old_pos = curr_pos.clone();

        // get new pos
        match dir {
            Direction::Up => { curr_pos.1 -= 1; },
            Direction::Right => { curr_pos.0 += 1; },
            Direction::Down => { curr_pos.1 += 1; },
            Direction::Left => { curr_pos.0 -= 1; },
        }

        let new_pos = matrix.get_with_pos(curr_pos);

        if new_pos.is_none() {
            // guard is outside array, we finish up and break
            break;
        }

        // otherwise we got new pos, lets check if it is blocked
        if new_pos.unwrap() == '#' {
            // we are blocked, we change dir and continue, so we do not count it as a step
            dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            // reset current pos, to old pos, so we are back at old step
            curr_pos = old_pos;
            continue;
        }

        // if we are here, then it is a valid move, we push the position to path
        path.insert(curr_pos);
    }

    // clean path to keep only unique parts
    // path.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

    let sum = path.iter().count(); // +1 to count start pos

    // debug, draw the path
    for pos in path {
        matrix.set(pos.0 as usize, pos.1 as usize, 'X');
    }
    matrix.print();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
                assert_eq!("41", process(input)?);
                Ok(())
    }
}