use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Default, Clone, Hash, Eq)]
struct Pos {
    pub x: i32,
    pub y: i32
}

impl PartialEq for Pos {
    fn eq(&self, other: &Pos) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    // hashmap to store all positions of a key
    let mut hmap: HashMap<char, Vec<Pos>> = HashMap::new();

    // parse input into matrix
    let mut matrix = Matrix::new(input);

    // parse all positions of matrix and if not "." we add it to hashmap
    for (y, row) in matrix.rows.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col != '.' {
                // push position to hashmap
                let pos = Pos {x: x as i32, y: y as i32};
                hmap.entry(*col).and_modify(|e| e.push(pos.clone())).or_insert(vec![pos.clone()]);
            }
        }
    }

    let mut antinodes_pos: HashSet<Pos> = HashSet::new();

    // go through each values in keys
    for (_, val) in hmap.iter() {
        // for each key, we go through each value, and check if we have positions with any other value
        for p in val.iter() {
            // go through all other members of val per val
            for p2 in val.iter() {
                // if ourself, we skip
                if p == p2 {
                    continue;
                }

                // get x and y difference between p and p2
                let x_diff = p2.x - p.x;
                let y_diff = p2.y - p.y;

                let antipos = Pos {x: p2.x + x_diff, y: p2.y + y_diff};

                match matrix.get(antipos.x, antipos.y) {
                    Some(c) => {antinodes_pos.insert(antipos);},
                    None => {continue}
                }
            }
        }
    }

    let sum: u64 = antinodes_pos.len() as u64;

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
                assert_eq!("14", process(input)?);
                Ok(())
    }
}