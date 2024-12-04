
#[derive(Debug, Default)]
struct Matrix {
    pub rows: Vec<Vec<char>>
}
impl Matrix {
    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        Some(*self.rows.get(row)?.get(col)?)
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

pub fn count_xmas(m: &Matrix, x: usize, y: usize, val: char) -> anyhow::Result<(u64, Vec<(usize, usize, char)>)> {

    let mut sum = 0;
    let mut possible_options: Vec<Vec<(usize, usize, char)>> = vec![];

    // we got the position of a X. Now we have to check all possibilities from here if XMAS exists
    if x >= 1 && y >= 1 {
        possible_options.push(vec![(x-1, y-1, 'M'), (x+1, y-1, 'S'), (x-1, y+1, 'M'), (x+1, y+1, 'S')]);
        possible_options.push(vec![(x-1, y-1, 'S'), (x+1, y-1, 'S'), (x-1, y+1, 'M'), (x+1, y+1, 'M')]);
        possible_options.push(vec![(x-1, y-1, 'M'), (x+1, y-1, 'M'), (x-1, y+1, 'S'), (x+1, y+1, 'S')]);
        possible_options.push(vec![(x-1, y-1, 'S'), (x+1, y-1, 'M'), (x-1, y+1, 'S'), (x+1, y+1, 'M')]);
    }

    // storage for valid options
    let mut valid_options: Vec<(usize, usize, char)> = vec![];

    // go through all options
    for option in possible_options.iter() {

        let mut valid = true;

        for (x, y, value) in option.iter() {
            if let Some(char) = m.get(*y, *x) {
                // we have an element on this pos, check if the right one?
                if char != *value {
                    valid = false;
                    break;
                }
            } else {
                valid = false;
                break;
            }
        }

        // if valid count +1
        if valid {
            valid_options.extend(option);
            valid_options.push((x, y, val)); // remember to include the x
            sum += 1;
        }
    }

    Ok((sum, valid_options))
}

pub fn process(input: &str) -> anyhow::Result<String> {
    // same as part 2, we are just looking for a different pattern that begins with A....

    // should make a grid then locate all X in the grid and do a outwards search from each X in all 4 directions until XMAS is found
    let mut m = Matrix::new(input);
    let mut sum: u64 = 0;
    // vector to hold valid chars
    let mut valids: Vec<(usize, usize, char)> = vec![];

    // go through all elements of matrix, if char == "X", we call the function to test which directions spell xmas
    for (y, row) in m.rows.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            // Got my x,y, val for each element
            if *val == 'A' {
                // check for valid options
                let (res_sum, mut valid_options) = count_xmas(&m, x, y, *val)?;
                sum += res_sum;
                valids.append(&mut valid_options);
            }
        }
    }

    // for debugging purpose lets print the resulting output
    m.set_all_to('.');
    for (x, y, val) in valids.iter() {
        m.set(*x, *y,*val);
    }
    m.print();


    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
                assert_eq!("9", process(input)?);
                Ok(())
    }
}