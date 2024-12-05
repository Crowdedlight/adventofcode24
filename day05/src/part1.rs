

pub fn process(input: &str) -> anyhow::Result<String> {

    let mut rules: Vec<(u64, u64)> = vec![];
    let mut pages: Vec<Vec<u64>> = vec![];

    // split input into rules and pages, line that is empty is delimiter
    let mut pages_input = false;
    for line in input.lines() {

        if line == "" {
            pages_input = true;
            continue;
        };

        if !pages_input {
            let (a, b) = line.split_once('|').unwrap();
            rules.push((a.parse()?, b.parse()?));
        } else {
            let pag = line.split(",").map(|x| x.parse().unwrap()).collect::<Vec<u64>>();
            pages.push(pag);
        }
    }

    let mut sum: u64 = 0;

    // run through each page
    for p in pages {
        let r = rules.clone();
        // let mut r_valid: Vec<(u64, u64)> = vec![];

        let mut valid_page = true;

        for rule in r {
            let a = p.iter().position(|&p| p == rule.0);
            let b = p.iter().position(|&p| p == rule.1);

            // if either is None, we skip the rule
            if a.is_none() || b.is_none() {
                continue;
            };

            // otherwise we check if the rule is invalid
            if a.unwrap() > b.unwrap() {
                valid_page = false;
                break;
            }
        }
        // if valid, we take median number and add to sum
        if valid_page {
            let middle = p.len() / 2;
            sum += p[middle];
            // println!("Valid vector: {:?}, mid_index: {}, middle element: {}", p, middle, p[middle]);
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
                assert_eq!("143", process(input)?);
                Ok(())
    }
}