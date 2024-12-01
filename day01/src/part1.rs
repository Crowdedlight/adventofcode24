pub fn process(input: &str) -> anyhow::Result<String> {

    let mut a_list: Vec<u32> = vec![];
    let mut b_list: Vec<u32> = vec![];

    // split into a and b lists
    for line in input.lines() {
        let (a, b) = line.split_once(' ').unwrap();
        a_list.push(a.trim().parse()?);
        b_list.push(b.trim().parse()?);
    }

    // sort both in increasing order
    a_list.sort();
    b_list.sort();

    // can we just subtract each list with eachother with abs() and sum the resulting list?
    let res: Vec<u32> = a_list.into_iter().zip(b_list).map(|(a, b)| a.abs_diff(b)).collect();
    Ok(res.iter().sum::<u32>().to_string())
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
                assert_eq!("11", process(input)?);
                Ok(())
    }
}