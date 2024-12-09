use std::fmt::Display;

#[derive(Debug, Clone)]
struct Memory {
    pub id: i64,
    pub size: usize
}

pub fn print_memory(input: &Vec<Memory>) {
    for i in input.iter() {

        for j in 0..i.size {
            if i.id == -1 {
                print!(".");
            } else {
                print!("{}", i.id);
            }
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

        let val: usize = c.to_digit(10).unwrap() as usize;

        // add freespace or file block
        match i % 2 != 0 {
            true => expanded.push(Memory {id: -1, size: val}),
            false => expanded.push(Memory {id: curr_id, size: val}),
        }

        // only update id if we have done files
        if i % 2 == 0 {
            curr_id += 1;
        }
    }
    // print_memory(&expanded);

    // move memory, easiest is to go from both ends, find first free space, then move last file to it
    let mut back_index = expanded.len();

    for i in (0..expanded.len()).rev() {
        // find first element
        if expanded[i].id != -1 {
            // go from front to end to find a free spot
            for j in 0..i {
                if expanded[j].id == -1 && expanded[j].size >= expanded[i].size {
                    // move memory
                    expanded.swap(i, j);

                    // if size is not excatly equal, we need to insert leftover freespace
                    let size_diff = expanded[j].size.abs_diff(expanded[i].size);
                    if size_diff > 0 {
                        // resize the space in the now i, index to be size of swap
                        expanded[i].size = expanded[j].size;

                        // insert block at i index
                        expanded.insert(j+1, Memory {id: -1, size: size_diff});
                    }

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
    let mut sum = 0u64;
    let mut index = 0u64;
    for val in expanded.iter() {
        if val.id != -1 {
            // have to do it for each position included in this memory
            for j in 0..val.size {
                sum += index as u64 * val.id as u64;
                index += 1;
            }
        } else {
            index += val.size as u64;
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}