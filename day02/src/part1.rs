pub fn process(input: &str) -> anyhow::Result<String> {

    // analyze each line. Levels has to be either always increasing or decreasing
    // jump between levels has to be 1 or at most 3, to be safe
    let mut count: u64 = 0;
    for line in input.lines() {
        if is_safe(line)? {
            count += 1;
        }
    }

    Ok(count.to_string())
}

pub fn is_safe(s: &str) -> anyhow::Result<bool> {

    let levels = s.split_whitespace().map(|e| e.parse::<u64>().unwrap()).collect::<Vec<_>>();

    // check if increasing or decreasing, we can just check if vector is sorted
    let increasing_safe = levels.is_sorted_by(|a, b| a < b && a.abs_diff(*b) <= 3 && a.abs_diff(*b) >= 1);
    let decreasing_safe = levels.is_sorted_by(|a, b| a > b && a.abs_diff(*b) <= 3 && a.abs_diff(*b) >= 1);

    Ok(increasing_safe || decreasing_safe)
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
                assert_eq!("2", process(input)?);
                Ok(())
    }
}