use regex::Regex;

#[derive(Debug)]
struct Instruction {
    input: String,
    a: Option<u64>,
    b: Option<u64>,
    sum: u64
}

impl Instruction {
    pub fn new(input: String, a: &str, b: &str) -> Instruction {

        // parse a/b
        let a_num = a.parse::<u64>().ok();
        let b_num = b.parse::<u64>().ok();
        //
        // let mut sum = None;
        // if let (Some(a_num), Some(b_num)) = (a_num, b_num) {
        //     sum = Some(a_num * b_num);
        // }

        Instruction { a: a_num, b: b_num, input, sum: a_num.unwrap_or(0) * b_num.unwrap_or(0) }
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"(mul\((?<a>\d+),(?<b>\d+)\)|(don't\(\))|(do\(\)))").unwrap();
    let re_mul = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    let mut instructions: Vec<Instruction> = vec![];

    let mut mul_enabled = true;

    println!("DEBUG ----------------");

    for cap in re.find_iter(input) {

        let str_match = cap.as_str();


        println!("cap: {}", str_match);

        if str_match == "don't()" {
            mul_enabled = false;
            continue;
        } else if str_match == "do()" {
            mul_enabled = true;
            continue;
        } else if str_match.contains("mul") && mul_enabled {
            let mul_cap = re_mul.captures(str_match).unwrap();

            instructions.push(Instruction::new(str_match.to_string(), &mul_cap["a"], &mul_cap["b"]));
        }
    }
    println!("----------------------");

    let res: u64 = instructions.iter().map(|v| v.sum).sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
                assert_eq!("48", process(input)?);
                Ok(())
    }
}