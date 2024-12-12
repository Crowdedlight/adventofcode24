
struct Stone {
    engraving: u64
}

pub fn process(input: &str) -> anyhow::Result<String> {

    // parse input
    let mut stones: Vec<u64> = input.split_whitespace().map(|c| c.parse::<u64>().unwrap()).collect();

    for n in 0..25 {

        let mut stone_copy: Vec<u64> = vec![];

        for (i, stone) in stones.iter().enumerate() {
            // the three rules
            if *stone == 0 {
                stone_copy.push(1);
                continue;
            }

            // even number is split
            if stone.to_string().len() % 2 == 0 {
                // splitting stones, get the new values
                let num_string = stone.to_string();
                let (a, b) = num_string.split_at(num_string.len() / 2);

                let new_stone_left = a.parse::<u64>()?;
                let stone_right = b.parse::<u64>()?;

                stone_copy.push(new_stone_left);
                stone_copy.push(stone_right);

                continue;
            }

            // if no other rules apply, this apply
            stone_copy.push(stone * 2024);
        }
        // update master list
        stones = stone_copy;

        // println!("Blink: {}, stones: {:?}", n, stones);
    }



    Ok(stones.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "125 17";
                assert_eq!("55312", process(input)?);
                Ok(())
    }
}