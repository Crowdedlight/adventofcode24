use std::fmt::Display;

#[derive(Debug, Clone)]
struct Memory {
    pub id: i64,
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub fn print_memory(input: &Vec<Memory>) {
    for i in input.iter() {
        if i.id != -1 {
            print!("{}", i.id);
        } else
        {
            print!(".");
        }
    }
    println!();
}

pub fn process(input: &str) -> anyhow::Result<String> {

    // what if we expand it first, then pop the back, and move to first . position
    let mut expanded: Vec<Memory> = vec![];

    let puzzle_in = input.lines().nth(0).unwrap();
    let mut curr_id = 0;

    for (i, c) in puzzle_in.chars().enumerate() {

        let val = c.to_digit(10).unwrap();
        for _ in 0..val {

            // if true, we are on every second position, which is free-space
            match i % 2 != 0 {
                true => {expanded.push(Memory { id: -1 });},
                false => {
                    // expanded.push(curr_id.to_string().chars().nth(0).unwrap());
                    expanded.push(Memory { id: curr_id});
                }
            }
        }
        // only update id if we have done files
        if i % 2 == 0 {
            curr_id += 1;
        }
    }
    // print_memory(&expanded);

    // move memory, easiest is to go from both ends, find first free space, then move last file to it
    let mut back_index = expanded.len();
    for (i, val) in expanded.clone().iter().enumerate() {
        // find first free space
        if val.id == -1 {
            // reverse from end
            for j in (i..back_index).rev() {
                if expanded[j].id != -1 {
                    // move memory
                    expanded.swap(i, j);
                    // expanded[i].id = expanded[j].id;
                    // expanded[j].id = -1;

                    back_index = j;
                    break;
                }
            }
        }
    }

    // print_memory(&expanded);

    // get sum
    let sum: u64 = expanded.iter().enumerate().filter_map(|(i, v)|
        match v.id {
            -1 => None,
            _ => Some(i as u64 * v.id as u64)
        }
    ).sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "2333133121414131402";
                assert_eq!("1928", process(input)?);
                Ok(())
    }
}