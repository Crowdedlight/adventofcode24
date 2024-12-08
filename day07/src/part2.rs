use rayon::iter::ParallelIterator;
use std::cmp::PartialEq;
use std::collections::VecDeque;
use std::fmt::{format, Display, Formatter};
use std::time::SystemTime;
use anyhow::Context;
use rayon::prelude::IntoParallelRefIterator;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Equation {
    pub numbers: VecDeque<u64>,
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

pub fn recursive_test(eq: &Equation) -> bool {

    let mut own_eq = eq.clone();

    let first_elem = own_eq.numbers.pop_front().unwrap();
    let res = recursive_runner(own_eq.sum, own_eq.numbers.clone(), first_elem);
    if res {
        //println!("Valid operations to get sum: {}", eq.sum);
        true
    } else {
        false
    }
}

pub fn recursive_runner(target_sum: u64, mut new_combo: VecDeque<u64>, curr_sum: u64) -> bool {
    // base case, list is == 1, and the value is our target
    if new_combo.is_empty() {
        return curr_sum == target_sum
    }

    // try all three operations
    let element = new_combo.pop_front().unwrap();

    let res_a = recursive_runner(target_sum, new_combo.clone(), curr_sum + element);
    let res_m = recursive_runner(target_sum, new_combo.clone(), curr_sum * element);
    let res_c = recursive_runner(target_sum, new_combo.clone(), format!("{}{}", curr_sum, element).parse::<u64>().unwrap());
    
    res_a || res_m || res_c
}

pub fn process(input: &str) -> anyhow::Result<String> {

    let start = SystemTime::now();
    
    let eqs = input.lines().map(Equation::new).collect::<Vec<Equation>>();


    let res: u64 = eqs.par_iter().filter(|&eq| recursive_test(eq)).map(|eq| eq.sum).sum();
    let stop = SystemTime::now();

    println!("Procesing time: {}ms", stop.duration_since(start).unwrap().as_millis());

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