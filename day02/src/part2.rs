
pub fn process(input: &str) -> anyhow::Result<String> {

    let mut count: u64 = 0;
    for line in input.lines() {
        if is_safe(line)? {
            count += 1;
        }
    }
    Ok(count.to_string())
}

pub fn is_safe(s: &str) -> anyhow::Result<bool> {

    let mut levels_increasing = s.split_whitespace().map(|e| e.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let mut levels_increasing_2 = levels_increasing.clone();
    
    let mut levels_decreasing = levels_increasing.clone();
    let mut levels_decreasing_2 = levels_increasing.clone();

    remove_first_bad_level(&mut levels_increasing, &mut levels_increasing_2, |a, b| a > b);
    remove_first_bad_level(&mut levels_decreasing, &mut levels_decreasing_2, |a, b| a < b);

    // check if increasing or decreasing, we can just check if vector is sorted
    let increasing_safe = levels_increasing.is_sorted_by(|a, b| a < b && a.abs_diff(*b) <= 3 && a.abs_diff(*b) >= 1);
    let increasing_safe_2 = levels_increasing_2.is_sorted_by(|a, b| a < b && a.abs_diff(*b) <= 3 && a.abs_diff(*b) >= 1);
    
    let decreasing_safe = levels_decreasing.is_sorted_by(|a, b| a > b && a.abs_diff(*b) <= 3 && a.abs_diff(*b) >= 1);
    let decreasing_safe_2 = levels_decreasing_2.is_sorted_by(|a, b| a > b && a.abs_diff(*b) <= 3 && a.abs_diff(*b) >= 1);

    Ok(increasing_safe || decreasing_safe || increasing_safe_2 || decreasing_safe_2)
}

pub fn remove_first_bad_level(levels: &mut Vec<u64>, levels_2: &mut Vec<u64>, compare: fn(&u64, &u64) -> bool) {

    for (idx, level) in levels.iter().enumerate() {
        let next_level = levels.get(idx + 1).unwrap_or(&0);
        // if 0, we have been through the vector
        if *next_level == 0 {
            return;
        }

        // test compare function to see if we are decreasing or increasing
        if compare(level, next_level) || level.abs_diff(*next_level) > 3 || level.abs_diff(*next_level) < 1 {
            // bruteforce method, we remove both current and next index, as either is the optimal removal
            levels.remove(idx);
            levels_2.remove(idx+1);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
                assert_eq!("4", process(input)?);
                Ok(())
    }
}