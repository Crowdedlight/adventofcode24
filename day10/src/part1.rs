#[derive(Debug, Clone, Default, Copy)]
struct Pos {
    x: usize,
    y: usize,
    val: char
}

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

    pub fn find_all(&self, input: char) -> Vec<Pos> {
        let mut res: Vec<Pos> = vec![];

        for (y, row) in self.rows.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if *val == input {
                    res.push(Pos{x, y, val: *val});
                }
            }
        }
        res
    }

    pub fn print(&self) {
        for row in self.rows.iter() {
            println!("{:?}", row.iter().collect::<String>());
        }
    }
}

pub fn recursive_path(map: &Matrix, mut path: Vec<Pos>, height: u32, x: i32, y: i32) -> Option<Vec<Pos>> {

    // base-case
    let new_pos = map.get(x, y).unwrap_or('.').to_digit(10)?;

    // if height is not +1, we return
    if new_pos != height {
        return None;
    }

    path.push(Pos{x: x as usize, y: y as usize, val: map.get(x, y)?});

    if height == 9 {
        // we push to path
        return Some(path);
    }

    // else we try all 4 options from here
    let up = recursive_path(map, path.clone(), height+1, x, y-1);
    let down = recursive_path(map, path.clone(), height+1, x, y+1);
    let right = recursive_path(map, path.clone(), height+1, x+1, y);
    let left = recursive_path(map, path.clone(), height+1, x-1, y);

    // append any Some vectors to my path vector, and return that?
    if let Some(mut up) = up {
        path.append(&mut up);
    }
    if let Some(mut down) = down {
        path.append(&mut down);
    }
    if let Some(mut right) = right {
        path.append(&mut right);
    }
    if let Some(mut left) = left {
        path.append(&mut left);
    }
    Some(path)
}

pub fn recursive_path_check(map: &Matrix, height: i32, dir: i32, target_height: u32, x: i32, y: i32) -> bool {

    // base-cases
    let new_pos = map.get(x, y).unwrap_or('.').to_digit(10);

    // if new_pos == 9, we return true, if None, we return false
    if new_pos.is_none() {
        return false;
    }

    if new_pos.unwrap() == target_height {
        // we push to path
        return true;
    }

    // if height is not +1, we return
    if height != height + dir {
        return false;
    }

    let up = recursive_path_check(map, height+dir, dir, target_height, x, y-1);
    let down = recursive_path_check(map, height+dir, dir, target_height, x, y+1);
    let right = recursive_path_check(map, height+dir, dir, target_height, x+1, y);
    let left = recursive_path_check(map, height+dir, dir, target_height, x-1, y);

    // todo might not work as nice.... not done
    true
}

pub fn process(input: &str) -> anyhow::Result<String> {

    // load as matrix
    let mut matrix = Matrix::new(input);

    // get all start points (Could do 9 instead and work backwards, if needed)
    let start_points = matrix.find_all('0');

    println!("Start points: {:?}", start_points);

    let mut paths: Vec<Vec<Pos>> = vec![];
    // recursion to get all paths
    for point in start_points {
        if let Some(p) = recursive_path(&matrix, vec![point], 0, point.x as i32, point.y as i32) {
            paths.push(p);
        }
    }

    let mut sum = 0u64;

    // for each path, count trailheads
    for ps in paths.iter() {
        let mut m_debug = matrix.clone();
        m_debug.set_all_to('.');

        println!();
        for p in ps.iter() {
            m_debug.set(p.x, p.y, p.val);
        }
        m_debug.print();

        // count sum
        let endings = m_debug.find_all('9').iter().count();
        println!("Trailheads count: {}", endings);
        sum += endings as u64;
    }
    
    println!("paths: {}", paths.len());

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
                assert_eq!("36", process(input)?);
                Ok(())
    }
}