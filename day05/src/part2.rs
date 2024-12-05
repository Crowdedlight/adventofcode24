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
    let mut p_invalid: Vec<(Vec<u64>, Vec<(u64, u64)>)> = vec![];

    // run through each page
    for p in pages {
        let mut r_valid: Vec<(u64, u64)> = vec![];
        let mut valid_page = true;

        for rule in rules.iter() {
            let a = p.iter().position(|&p| p == rule.0);
            let b = p.iter().position(|&p| p == rule.1);

            // if either is None, we skip the rule
            if a.is_none() || b.is_none() {
                continue;
            };

            // add rule to array
            r_valid.push(rule.clone());

            // otherwise we check if the rule is invalid
            if a.unwrap() > b.unwrap() {
                valid_page = false;
            }
        }

        // push invalid pages and all their rules
        if !valid_page {
            p_invalid.push((p, r_valid));
        }
    }

    // we got all the invalid pages
    for p in p_invalid.iter() {
        println!("invalid page: {:?}, rules: {:?}", p.0, p.1);
    }

    // TODO figure out how we sort them according to the rules, need a good idea to not get gnarly...
    for p in p_invalid.iter() {
        // we got all rules for this page, and the page, how do we sort it, can we pass a closure function
        //  that sort the rules?

        // swap rules? if current position is invalid, swap it with the one it should be?
        // continue until sorted?

        let mut sorted = false;
        let mut sorted_page = p.0.clone();
        // todo, while loop until sorted == true, go through all rules, when finding an error we swap positions
        //  then either continues or start over
        while !sorted {
            // check each rule
            sorted = true;
            for rule in p.1.iter() {
                let a = sorted_page.iter().position(|&p| p == rule.0).unwrap();
                let b = sorted_page.iter().position(|&p| p == rule.1).unwrap();

                if a > b {
                    // rule not adhered to, swap em
                    sorted_page.swap(a, b);
                    sorted = false;
                }
            }
        }

        // sorted, we can sum middle position
        let middle = sorted_page.len() / 2;
        sum += sorted_page[middle];
        println!("Valid vector: {:?}, mid_index: {}, middle element: {}", sorted_page, middle, sorted_page[middle]);
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
                assert_eq!("123", process(input)?);
                Ok(())
    }
}