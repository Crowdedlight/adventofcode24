
struct Machine {
    target_x: usize,
    target_y: usize,
    a: (usize, usize),
    b: (usize, usize),
}
impl Machine {
    fn new(target_x: usize, target_y: usize, a: (usize, usize), b: (usize, usize)) -> Self {
        Self {target_x, target_y, a, b}
    }
}

// return press_a, press_b and sum
pub fn recursive_btn_press(machine: &Machine,  a_press: usize, b_press: usize) -> Option<(usize, usize, usize)> {

    let new_x = machine.a.0 * a_press + machine.b.0 * b_press;
    let new_y = machine.a.1 * a_press + machine.b.1 * b_press;

    // base class, check if out of bounds or we reached target
    if new_x == machine.target_x && new_y == machine.target_y {
        // found the goal, time to return
        return Some((a_press, b_press, a_press*3 + b_press*1))
    }

    // check if no solution exists
    if new_x > machine.target_x || new_y > machine.target_y || a_press > 100 || b_press > 100 {
        return None;
    }

    // otherwise we try both new options
    let option_a = recursive_btn_press(machine, a_press+1, b_press);
    let option_b = recursive_btn_press(machine, a_press, b_press+1);

    // todo what if both A and B is valid? check which one is cheapest and return that one?
    if let Some(a) = option_a {
        return option_a
    }

    todo!()
}

pub fn process(input: &str) -> anyhow::Result<String> {

    // A costs 3 tokens
    // B cost 1 token
    // Max press per button is 100

    // todo, could do recursive. Every step, I try to both do A or B push, until we hit, or go over the target (x,y), or go over 100 presses on either a/b in total
    //  keeping track of button presses and returns the sum of (a,b) button press and price?

    // todo minimalisation problem? Plot both as a curve and see if they intersect or local minima?
    // plot with x,y
    // example 1: f(1) = 1*94, 1*34,
    // example 1: f(2) = 2*94, 2*34,
    // example 1: f(3) = 3*94, 3*34,
    // Answer: A: 80, B: 40

    // formula is: min_token = x*a + y*b
    // min_token_x = 80*a + 34*b
    // min_token_x = x * a.x + y * b.x
    // min_token_y = x * a.y + y * b.y

    // some machines have no combo that allows you to win. Use the 100 per button limit here?
    // can we be clever here and assume that if we do modula of a, b, and combined, and none of them goes up into target position, then we will never be able to hit it?
    //
    // Button A: X+26, Y+66
    // Button B: X+67, Y+21
    // Prize: X=12748, Y=12176
    // 12748 % 26 = 8
    // 12748 % 67 = 18
    // 12748 % (26+67) = 7

    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    // 8400 % 94 = 34
    // 8400 % 22 = 18
    // 8400 % (94+22) = 48



    // find the fewest tokens you would spend to win all possible prices


    // sum the tokens per machine as that is our answer to minimum spend of tokens to wind all possible prices

    todo!()
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
                assert_eq!("480", process(input)?);
                Ok(())
    }
}