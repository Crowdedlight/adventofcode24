use std::collections::HashMap;

pub fn process(input: &str) -> anyhow::Result<String> {

    let mut a_list: Vec<u32> = vec![];
    // hashmap makes more sense for this problem
    let mut map: HashMap<u32, u32> = HashMap::new();

    // split into a and b lists
    for line in input.lines() {
        let (a, b) = line.split_once(' ').unwrap();
        a_list.push(a.trim().parse()?);

        // get or create key
        let b_val = b.trim().parse()?;
        map.entry(b_val).and_modify(|v| *v += 1).or_insert(1);
    }

    // make right list a hashmap with the number as key and the value the count
    let mut sum: u64 = 0;
    for val in a_list {
        // get count for key in map, if key doesn't exist, we return 0
        let count = map.get(&val).unwrap_or(&0);
        sum += *count as u64 * val as u64;
    }
    
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
                assert_eq!("31", process(input)?);
                Ok(())
    }
}