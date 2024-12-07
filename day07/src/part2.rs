use std::cmp::PartialEq;
use std::fmt::{format, Display, Formatter};

#[derive(Clone, Debug, PartialOrd, PartialEq)]
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
    Concat = 3
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let str = match self {
            Operation::Add => "+",
            Operation::Multiply => "*",
            Operation::Concat => "||"
        };

        write!(f, "{}", str)
    }
}

pub fn find_combination_bruteforce(mut equation: &Equation) -> Result<bool, anyhow::Error> {

    // bruteforce method, we find all possible combinations of + and *
    let mut own_eq = equation.clone();

    // get all combinations
    let possible_combos = get_all_combinations(3, own_eq.numbers.len()-1);

    //println!("Possible Combos for : {:?}", equation.numbers);
    //for o in possible_combos.iter() {
    //    println!("{:?}", o);
    //}

    // run through all options and check if we hit sum
    for opt in possible_combos {

        let mut sum = own_eq.numbers[0];

        for i in 1..own_eq.numbers.len() {

            // calculate sum
            match opt[i-1] {
                Operation::Add => sum += own_eq.numbers[i],
                Operation::Multiply => sum *= own_eq.numbers[i],
                Operation::Concat => {
                    // concat, so we take sum and concat with current value
                    sum = format!("{}{}", sum, own_eq.numbers[i]).parse::<u64>()?;
                }
            }

            // early return if we are above target sum
            if sum > own_eq.sum {
                break;
            }

            // return as soon as we find one hitting targed sum
            if sum == own_eq.sum {
                print!("valid combo: {} = ", own_eq.sum);
                for (idx, num) in own_eq.numbers.iter().enumerate() {
                    if idx == 0 {
                        print!("{} ", num);
                    } else {
                        print!("{} {} ", opt[idx-1], num);
                    }
                }
                println!();
                
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
    let possible_set = vec![Operation::Add, Operation::Multiply, Operation::Concat];

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
        assert_eq!("11387", process(input)?);
        Ok(())
    }
}