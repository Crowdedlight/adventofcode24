use std::cmp::PartialEq;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Default)]
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
    pub fn get_with_pos(&self, pos: &Pos) -> Option<char> {
        self.get(pos.x, pos.y)
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

#[derive(Clone, Debug, Copy, Eq, PartialOrd)]
#[derive(Ord)]
struct Pos {
    pub x: i32,
    pub y: i32,
    pub dir: Direction,
}

impl Pos {
    pub fn new(x: i32, y: i32, dir: Direction) -> Self {
        Pos { x, y, dir }
    }

    pub fn is_equal_pos(&self, pos: &Pos) -> bool {
        self.x == pos.x && self.y == pos.y
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.dir == other.dir
    }
}

// pub fn calculate_path_loop(m: &Matrix, start_pos: Pos, new_block_pos: Pos) -> Result<Vec<Pos>, anyhow::Error> {
//     // main vars
//     let mut path: Vec<Pos> = vec![];
//     let mut dir = start_pos.dir.clone();
//     let mut curr_pos = start_pos.clone();
//
//     let mut m_map = m.clone();
//
//     // if given set new blocker
//     if let Some(p) = new_block_pos {
//         m_map.set(p.x as usize, p.y as usize, '#');
//     }
//
//
//     // include start pos
//     path.push(start_pos.clone());
//
//     // main loop, keep going until new pos is None
//     loop {
//         // clone pos to keep last pos in case new pos is not valid
//         let old_pos = curr_pos.clone();
//
//         // get new pos
//         match curr_pos.dir {
//             Direction::Up => { curr_pos.y -= 1; },
//             Direction::Right => { curr_pos.x += 1; },
//             Direction::Down => { curr_pos.y += 1; },
//             Direction::Left => { curr_pos.x -= 1; },
//         }
//
//         let new_pos = m_map.get_with_pos(&curr_pos);
//
//         if new_pos.is_none() {
//             // guard is outside array, we finish up and break
//             return if initial {
//                 Ok(path)
//             } else {
//                 Err(anyhow::anyhow!("no circular path"))
//             }
//         }
//
//         // circular also if we deadlock?
//         if path.contains(&curr_pos) && !initial {
//             // return Err(anyhow::anyhow!("no circular path"));
//             return Ok(path);
//         }
//
//         // otherwise we got new pos, lets check if it is blocked
//         if new_pos.unwrap() == '#' {
//             // we are blocked, we change dir and continue, so we do not count it as a step
//             dir = match curr_pos.dir {
//                 Direction::Up => Direction::Right,
//                 Direction::Right => Direction::Down,
//                 Direction::Down => Direction::Left,
//                 Direction::Left => Direction::Up,
//             };
//             // reset current pos, to old pos, so we are back at old step
//             curr_pos = old_pos;
//             // save new dir
//             curr_pos.dir = dir;
//             continue;
//         }
//
//         // if we are here, then it is a valid move, we push the position to path
//         path.push(curr_pos);
//     }
// }

pub fn calculate_path(m: &Matrix, start_pos: Pos, new_block_pos: Option<Pos>, initial: bool) -> Result<Vec<Pos>, anyhow::Error> {
    // main vars
    let mut path: Vec<Pos> = vec![];
    let mut dir = start_pos.dir.clone();
    let mut curr_pos = start_pos.clone();

    let mut m_map = m.clone();

    // if given set new blocker
    if let Some(p) = new_block_pos {
        m_map.set(p.x as usize, p.y as usize, '#');
    }


    // include start pos
    path.push(start_pos.clone());

    // main loop, keep going until new pos is None
    loop {
        // clone pos to keep last pos in case new pos is not valid
        let old_pos = curr_pos.clone();

        // get new pos
        match curr_pos.dir {
            Direction::Up => { curr_pos.y -= 1; },
            Direction::Right => { curr_pos.x += 1; },
            Direction::Down => { curr_pos.y += 1; },
            Direction::Left => { curr_pos.x -= 1; },
        }

        let new_pos = m_map.get_with_pos(&curr_pos);

        if new_pos.is_none() {
            // guard is outside array, we finish up and break
            return if initial {
                Ok(path)
            } else {
                Err(anyhow::anyhow!("no circular path"))
            }
        }

        // circular also if we deadlock?
        if path.contains(&curr_pos) && !initial {
            // return Err(anyhow::anyhow!("no circular path"));
            return Ok(path);
        }

        // otherwise we got new pos, lets check if it is blocked
        if new_pos.unwrap() == '#' {
            // we are blocked, we change dir and continue, so we do not count it as a step
            dir = match curr_pos.dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            // reset current pos, to old pos, so we are back at old step
            curr_pos = old_pos;
            // save new dir
            curr_pos.dir = dir;
            continue;
        }

        // if we are here, then it is a valid move, we push the position to path
        path.push(curr_pos);
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    // like part 1, but for each step in the path we also save dir

    // load the matrix
    let mut matrix = Matrix::new(input);

    // find stat location
    let mut start_pos: Pos = Pos::new(0, 0, Direction::Up);

    for (idx, row) in matrix.rows.iter().enumerate() {
        let pos = row.iter().position(|&c| c == '^');

        if pos.is_some() {
            start_pos = Pos::new(pos.unwrap() as i32, idx as i32, Direction::Up);
            break;
        }
    }

    let guard_path = calculate_path(&matrix, start_pos, None, true).unwrap();

    // print guard path
    // println!("Original Guard Path!");
    // let mut guard_matrix = matrix.clone();
    // for (i, p) in guard_path.iter().enumerate() {
    //
    //     let val = match p.dir {
    //         Direction::Up => '|',
    //         Direction::Right => '-',
    //         Direction::Down => '|',
    //         Direction::Left => '-',
    //     };
    //
    //     guard_matrix.set(p.x as usize, p.y as usize, val);
    // }
    // guard_matrix.print();

    // after having the full path we go through all positions and try to place a block on the next position. Then finish the new path until out of bounds or hitting a repeat position and direction

    // let mut circular_paths: Vec<(Vec<Pos>, Pos)> = vec![];
    // let mut m = matrix.clone();

    
    let mut result = guard_path.par_iter().filter(|&&path| calculate_path(&matrix, start_pos, Some(path), false).is_ok()).collect::<Vec<&Pos>>();
    
    // more readable non-parallel calculation
    // for pos in guard_path.par_iter() {
    //     // test if we become cirular if we put a block on the next pos
    //     // println!("Testing position {} out of {}...", num, guard_path.len());
    //
    //     // save old value at pos on matrix for reset purpose
    //     let old_val = m.get(pos.x, pos.y).unwrap();
    //
    //     // set current pos as # on matrix
    //     m.set(pos.x as usize, pos.y as usize, '#');
    //
    //     let res = calculate_path(&m, start_pos, false);
    //
    //     // reset matrix pos
    //     m.set(pos.x as usize, pos.y as usize, old_val);
    //
    //     if let Ok(res) = res {
    //         circular_paths.push((res, *pos));
    //     }
    // }

    // debug, print circular paths
    // println!();
    // println!("Circular paths!");
    // for c in circular_paths.iter() {
    //     let mut print_matrix = matrix.clone();
    //     for (i, p) in c.0.iter().enumerate() {
    //
    //         let val = match p.dir {
    //             Direction::Up => { if i == 0 {'^'} else {'|'}},
    //             Direction::Right => { if i == 0 {'>'} else {'-'}},
    //             Direction::Down => { if i == 0 {'v'} else {'|'}},
    //             Direction::Left => { if i == 0 {'<'} else {'-'}},
    //         };
    //
    //         print_matrix.set(p.x as usize, p.y as usize, val);
    //     }
    //     print_matrix.set(c.1.x as usize, c.1.y as usize, 'O');
    //     print_matrix.print();
    //     println!();
    //     println!();
    // }

    // dedup for unique
    // let mut vec_obs = circular_paths.iter().map(|p| p.1).collect::<Vec<Pos>>();
    // let mut vec_obs = result.iter().map(|p| p.1).collect::<Vec<Pos>>();
    result.sort();
    result.dedup_by(|a, b| {a.is_equal_pos(b)});

    // let sum = vec_obs.iter().count();
    let sum = result.len();

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
                assert_eq!("6", process(input)?);
                Ok(())
    }
}