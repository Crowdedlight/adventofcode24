use std::cmp::PartialEq;

struct Equation {
    pub numbers: Vec<u64>,
    pub sum: u64
}

impl Equation {
    pub fn new(input: &str) -> Equation {
        // split on :
        let (sum, rest) = input.split_once(":").unwrap();
        let numbers = rest.trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();

        Equation { numbers, sum: sum.trim().parse::<u64>().unwrap()}

    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Operation {
    Add = 1,
    Multiply = 2,
    Error = 3
}

pub fn find_combination_bruteforce(equation: &Equation) -> Result<bool, anyhow::Error> {

    // bruteforce method, we find all possible combinations of + and *
    let elements = equation.numbers.len();

    let opt = [Operation::Add, Operation::Multiply];

    // get all combinations
    let possible_combos = get_all_combinations(2, equation.numbers.len()-1);

    //println!("Possible Combos");
    //for o in possible_combos.iter() {
    //    println!("{:?}", o);
    //}

    // run through all options and check if we hit sum
    for opt in possible_combos {
        let mut sum = equation.numbers[0];
        for i in 0..equation.numbers.len()-1 {
            // calculate sum
            if opt[i] == Operation::Add {
                sum += equation.numbers[i+1];
            } else {
                sum *= equation.numbers[i+1];
            }

            // early return if we are above target sum
            if sum > equation.sum {
                break;
            }

            // return as soon as we find one hitting targed sum
            if sum == equation.sum {
                //println!("valid combo: {:?}", opt);
                return Ok(true);
            }
        }
    }
    Ok(false)
}

// k == size of combinations, n == possible options per element
pub fn get_all_combinations(n: usize, k: usize) -> Vec<Vec<Operation>> {
    let mut combinations: Vec<Vec<Operation>> = vec![];
    let mut current_combination: Vec<Operation> = vec![];
    let possible_set = vec![Operation::Add, Operation::Multiply];

    all_combinations_recursive(&mut combinations, &mut current_combination, &possible_set, n, k);
    combinations
}

pub fn all_combinations_recursive(combinations: &mut Vec<Vec<Operation>>, new_combo: &mut Vec<Operation>, possible_set: &Vec<Operation>, n: usize, k: usize) {
    // base case, k is 0
    if k == 0 {
        combinations.push(new_combo.clone());
        return;
    }

    // One by one add all operations
    // from set and recursively
    // call for k equals to k-1
    for i in 0..n {

        // take current value and add the next one
        let mut new_c = new_combo.clone();
        new_c.push(possible_set[i]);

        // call recursively
        all_combinations_recursive(combinations, &mut new_c, possible_set, n, k - 1);
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {

    let eqs = input.lines().map(Equation::new).collect::<Vec<Equation>>();

    // for each equation, find the combination of * and +, that gives matches sum
    //for eq in eqs.iter() {
    //    let res = find_combination_bruteforce(&eq)?;
    //}

    let res: u64 = eqs.iter().filter(|eq| find_combination_bruteforce(eq).unwrap_or(false)).map(|eq| eq.sum).sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
                assert_eq!("3749", process(input)?);
                Ok(())
    }
}