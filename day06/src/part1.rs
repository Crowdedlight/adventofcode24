pub fn process(input: &str) -> anyhow::Result<String> {

    // matrix again, use a get function returning Option and input of i32, with < 0, checks.
    // so we can just get the start position and then move until obstacles or outside of map
    
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
                assert_eq!("4361", process(input)?);
                Ok(())
    }
}