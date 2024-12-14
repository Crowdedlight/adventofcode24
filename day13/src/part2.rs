use anyhow::anyhow;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    target_x: u64,
    target_y: u64,
    a: (u64, u64),
    b: (u64, u64),
}
impl Machine {
    fn new(target_x: u64, target_y: u64, a: (u64, u64), b: (u64, u64)) -> Self {
        Self {target_x, target_y, a, b}
    }
}

// return press_a, press_b and sum
pub fn recursive_btn_press(machine: &Machine,  a_press: u64, b_press: u64) -> Option<(u64, u64, u64)> {

    let new_x = machine.a.0 * a_press + machine.b.0 * b_press;
    let new_y = machine.a.1 * a_press + machine.b.1 * b_press;

    // base class, check if out of bounds or we reached target
    if new_x == machine.target_x && new_y == machine.target_y {
        // println!("FOUND: tgt: ({}, {}), curr: ({}, {}), a_press: {}, b_press: {}", machine.target_x, machine.target_y, new_x, new_y, a_press, b_press);
        // found the goal, time to return
        return Some((a_press, b_press, a_press*3 + b_press*1))

    }

    // check if no solution exists
    if new_x > machine.target_x || new_y > machine.target_y || a_press > 100 || b_press > 100 {
        // println!("NONE: tgt: ({}, {}), curr: ({}, {}), a_press: {}, b_press: {}", machine.target_x, machine.target_y, new_x, new_y, a_press, b_press);
        return None;
    }

    println!("CONTINUE: tgt: ({}, {}), curr: ({}, {}), a_press: {}, b_press: {}", machine.target_x, machine.target_y, new_x, new_y, a_press, b_press);

    // otherwise we try both new options
    let option_a = recursive_btn_press(machine, a_press+1, b_press);
    let option_b = recursive_btn_press(machine, a_press, b_press+1);

    println!("RETURNING: tgt: ({}, {}), curr: ({}, {}), a_press: {}, b_press: {}", machine.target_x, machine.target_y, new_x, new_y, a_press, b_press);

    // if both exists, we find cheapest and return, otherwise we return the one that is some, or none
    if option_a.is_none() && option_b.is_some() {
        option_b
    } else if option_b.is_none() && option_a.is_some() {
        option_a
    } else if option_a.is_some() && option_b.is_some() {
        // return cheapest
        match option_a?.2 < option_b?.2 {
            true => {option_a},
            false => {option_b}
        }
    } else {
        println!("DEADEND: tgt: ({}, {}), curr: ({}, {}), a_press: {}, b_press: {}", machine.target_x, machine.target_y, new_x, new_y, a_press, b_press);
        None
    }
}

pub fn find_solution(m: &Machine) -> anyhow::Result<(u64, u64)> {

    // solve by a substitution to find b
    let top = (m.a.0 * m.target_y) as f64 - (m.a.1 * m.target_x) as f64;
    let bottom = (m.a.0 * m.b.1) as f64 - (m.b.0 * m.a.1) as f64;

    let b = top / bottom;
    let a = (m.target_x as f64 - b * m.b.0 as f64) / m.a.0 as f64;

    if a < 0.0 || b < 0.0 || a.fract() != 0.0 || b.fract() != 0.0 {
        return Err(anyhow!("no solution"))
    }

    // println!("a0: {}, b0: {}", a, b);

    Ok((a as u64, b as u64))
}

pub fn process(input: &str) -> anyhow::Result<String> {

    let mut machines : Vec<Machine> = vec![];

    let mut re_btn = Regex::new(r"(?:X\+(?<x>\d+)).+(?:Y\+(?<y>\d+))").unwrap();
    let mut re_tgt = Regex::new(r"(?:X=(?<x>\d+)).+(?:Y=(?<y>\d+))").unwrap();

    // input is split with newline newline
    let pat = match (cfg!(test), cfg!(windows)) {
        (false, true) => "\r\n\r\n",
        (_, false) => "\n\n",
        _ => "\n\n"
    };

    for l in input.split(pat) {

        // input here is format:
        // Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400

        let mut a_x = 0u64;
        let mut a_y = 0u64;
        let mut b_x = 0u64;
        let mut b_y = 0u64;
        let mut tgt_x = 0u64;
        let mut tgt_y = 0u64;

        for (i, line) in l.lines().enumerate() {
            if i == 0 {
                // btn a
                let cap = re_btn.captures(line).unwrap();
                a_x = cap["x"].parse::<u64>()?;
                a_y = cap["y"].parse::<u64>()?;
            } else if i == 1{
                // btn b
                let cap = re_btn.captures(line).unwrap();
                b_x = cap["x"].parse::<u64>()?;
                b_y = cap["y"].parse::<u64>()?;
            } else {
                // tgt input
                let cap = re_tgt.captures(line).unwrap();
                tgt_x = cap["x"].parse::<u64>()?;
                tgt_y = cap["y"].parse::<u64>()?;
            }
        }
        machines.push(Machine::new(tgt_x + 10000000000000, tgt_y + 10000000000000, (a_x, a_y), (b_x, b_y)));
    }

    let mut sum = 0u64;

    // A costs 3 tokens
    // B cost 1 token
    // Max press per button is 100

    // go through all machines and run recursive
    for (i, m) in machines.iter().enumerate() {

        println!("Running machine {}: {:?}...", i+1, m);

        // its pure math, 2 equations with 2 unknowns, we do the math and we got an equation for each
        if let Ok((a, b)) = find_solution(m) {
            println!("Solution - a_press: {}, b_press: {}, tokens: {}, tgt: ({}, {}), res: ({}, {})", a, b, a*3 + b, m.target_x, m.target_y, a*m.a.0 + b*m.b.0, a*m.a.1 + b*m.b.1);
            sum += a*3 + b;
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("875318608908", process(input)?);
        Ok(())
    }
}