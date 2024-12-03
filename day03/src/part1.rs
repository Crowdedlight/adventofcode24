use regex::Regex;

#[derive(Debug)]
struct Mul {
    input: String,
    a: u64,
    b: u64,
    sum: u64
}

impl Mul {
    pub fn new(s: &str, a: &str, b: &str) -> Mul {
        let a_num = a.parse::<u64>().unwrap();
        let b_num = b.parse::<u64>().unwrap();

        Mul {input: String::from(s), a:a_num, b:b_num, sum: (a_num * b_num)}
    }
}


pub fn process(input: &str) -> anyhow::Result<String> {

    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    let mut instructions: Vec<Mul> = vec![];

    for (cap, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        println!("cap: {}, a: {}, b: {}", cap, a, b);

        instructions.push(Mul::new(cap, a, b));
    }

    let res: u64 = instructions.iter().map(|v| v.sum).sum();


    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
                assert_eq!("161", process(input)?);
                Ok(())
    }
}