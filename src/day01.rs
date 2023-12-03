/*
--- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take
a look. The Elves have even given you a map; on it, they've used stars to mark
the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you
need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day
in the Advent calendar; the second puzzle is unlocked when you complete the
first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough")
and where they're even sending you ("the sky") and why your map looks mostly
blank ("you sure ask a lot of questions") and hang on did you just say the sky
("of course, where do you think snow comes from") when you realize that the
Elves are already loading you into a trebuchet ("please hold still, we need to
strap you in").

As they're making the final adjustments, they discover that their calibration
document (your puzzle input) has been amended by a very young Elf who was
apparently just excited to show off her art skills. Consequently, the Elves are
having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line
originally contained a specific calibration value that the Elves now need to
recover. On each line, the calibration value can be found by combining the first
digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and
77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the
calibration values?
*/

/*
--- Part Two ---
Your calculation isn't quite right. It looks like some of the digits are
actually spelled out with letters: one, two, three, four, five, six, seven,
eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last
digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
Adding these together produces 281.

What is the sum of all of the calibration values?
*/

use anyhow::bail;
use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

pub fn part1(input: &str) -> anyhow::Result<u32> {
    let mut total = 0;
    for line in input.lines() {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let Some(first) = digits.next() else {
            bail!("no digits in line {}", line);
        };
        let last = digits.last().unwrap_or(first);
        total += 10 * first + last;
    }
    Ok(total)
}

pub fn part2(input: &str) -> anyhow::Result<u32> {
    let mut total: u32 = 0;
    for line in input.lines().map(|l| l.trim()) {
        let digits = parse_part2_line(line);
        let Some(first) = digits.first().copied() else {
            bail!("no digits in line {}", line);
        };
        let last = digits.last().copied().unwrap_or(first);
        total += 10 * (first as u32) + (last as u32);
    }
    Ok(total)
}

// This is not a normal parser. The inputs can overlap. (e.g., "twone" should
// yield 2 from the "two" and 1 from the "one"). Rather than using normal nom
// combinators, we'll just manually iterate over every starting point.  Strictly
// speaking we don't actually need to parse the whole line. It would be faster
// to find the first digit, then skip to the end and work backwards to find the
// last digit.
fn parse_part2_line(input: &str) -> Vec<u8> {
    (0..input.len())
        .filter_map(|i| {
            let (_, d) = digit_parser(&input[i..]).ok()?;
            Some(d)
        })
        .collect()
}

fn digit_parser(input: &str) -> IResult<&str, u8> {
    alt((
        value(0, tag("0")),
        value(1, tag("1")),
        value(2, tag("2")),
        value(3, tag("3")),
        value(4, tag("4")),
        value(5, tag("5")),
        value(6, tag("6")),
        value(7, tag("7")),
        value(8, tag("8")),
        value(9, tag("9")),
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "
        .trim();
        assert_eq!(part1(input).unwrap(), 142);
    }

    #[test]
    fn part1_real_input() {
        let input = std::fs::read_to_string("data/day01.input").unwrap();
        assert_eq!(part1(&input).unwrap(), 54331);
    }

    #[test]
    fn part2_sample_input() {
        let input = "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "
        .trim();
        assert_eq!(part2(input.trim()).unwrap(), 281);
    }

    #[test]
    fn part2_parser() {
        assert_eq!(parse_part2_line("twone"), vec![2, 1]);
    }

    #[test]
    fn part2_real_input() {
        let input = std::fs::read_to_string("data/day01.input").unwrap();
        assert_eq!(part2(&input).unwrap(), 54518);
    }
}
