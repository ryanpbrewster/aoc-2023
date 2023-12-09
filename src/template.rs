pub fn part1(input: &str) -> anyhow::Result<i32> {
    parse_input(input)
}

fn parse_input(_input: &str) -> anyhow::Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "
        hello world
    ";

    #[test]
    fn parser_smoke_test() {
        assert_eq!(parse_input(SAMPLE_INPUT).unwrap(), 0);
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT).unwrap(), 0);
    }
}
