use std::collections::{HashMap, HashSet};

pub fn process(input: &str) -> anyhow::Result<String> {

    // parse input
    let mut stones: Vec<u64> = input.split_whitespace().map(|c| c.parse::<u64>().unwrap()).collect();

    let mut hashmap: HashMap<u64, u64> = HashMap::new();

    for stone in stones {
        hashmap.entry(stone).and_modify(|s| *s += 1).or_insert(1);
    }

    for n in 0..75 {

        // println!("Blink: {}", n+1);

        for (key, num) in hashmap.clone().iter() {
            // if val is 0, we skip as it technically does not exist
            if *num == 0 {
                continue;
            }

            // the three rules
            if *key == 0 {
                hashmap.entry(1).and_modify(|s| *s += *num).or_insert(*num);

                // set current key to 0
                hashmap.entry(*key).and_modify(|s| *s -= *num);
                continue;
            }

            // even number is split
            if key.to_string().len() % 2 == 0 {
                // splitting stones, get the new values
                let num_string = key.to_string();
                let (a, b) = num_string.split_at(num_string.len() / 2);

                let new_stone_left = a.parse::<u64>()?;
                let stone_right = b.parse::<u64>()?;

                hashmap.entry(new_stone_left).and_modify(|s| *s += *num).or_insert(*num);
                hashmap.entry(stone_right).and_modify(|s| *s += *num).or_insert(*num);

                hashmap.entry(*key).and_modify(|s| *s -= *num).or_insert(0);
                continue;
            }

            // if no other rules apply, this apply
            let new_stone = key * 2024;
            hashmap.entry(new_stone).and_modify(|s| *s += *num).or_insert(*num);

            hashmap.entry(*key).and_modify(|s| *s -= *num).or_insert(0);
        }

        // println!("Blink: {}, stones: {:?}, diff: {}", n, stone_copy.len(), stone_copy.len() - stones.len());
        // println!("Blink_map: {:?}", hashmap);
        // println!("{:?}", hashmap.iter().map(|(key, val)| *val).sum::<u64>());
    }
    // println!("{:?}", hashmap.iter().map(|(key, val)| *val).collect());

    let res = hashmap.iter().map(|(key, val)| *val).sum::<u64>();

    Ok(res.to_string())
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