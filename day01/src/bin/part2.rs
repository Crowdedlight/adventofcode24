use anyhow::Context;
use day01::part2::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}