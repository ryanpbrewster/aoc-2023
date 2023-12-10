/*
You ride the camel through the sandstorm and stop where the ghost's maps told
you to stop. The sandstorm subsequently subsides, somehow seeing you standing at
an oasis!

The camel goes to get some water and you stretch your neck. As you look up, you
discover what must be yet another giant floating island, this one made of metal!
That must be where the parts to fix the sand machines come from.

There's even a hang glider partially buried in the sand here; once the sun rises
and heats up the sand, you might be able to use the glider and the hot air to
get all the way up to the metal island!

While you wait for the sun to rise, you admire the oasis hidden here in the
middle of Desert Island. It must have a delicate ecosystem; you might as well
take some ecological readings while you wait. Maybe you can report any
environmental instabilities you find to someone so the oasis can be around for
the next sandstorm-worn traveler.

You pull out your handy Oasis And Sand Instability Sensor and analyze your
surroundings. The OASIS produces a report of many values and how they are
changing over time (your puzzle input). Each line in the report contains the
history of a single value. For example:

0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45

To best protect the oasis, your environmental report should include a prediction
of the next value in each history. To do this, start by making a new sequence
from the difference at each step of your history. If that sequence is not all
zeroes, repeat this process, using the sequence you just generated as the input
sequence. Once all of the values in your latest sequence are zeroes, you can
extrapolate what the next value of the original history should be.

In the above dataset, the first history is 0 3 6 9 12 15. Because the values increase by 3 each step, the first sequence of differences that you generate will be 3 3 3 3 3. Note that this sequence has one fewer value than the input sequence because at each step it considers two numbers from the input. Since these values aren't all zero, repeat the process: the values differ by 0 at each step, so the next sequence is 0 0 0 0. This means you have enough information to extrapolate the history! Visually, these sequences can be arranged like this:

0   3   6   9  12  15
  3   3   3   3   3
    0   0   0   0
To extrapolate, start by adding a new zero to the end of your list of zeroes; because the zeroes represent differences between the two values above them, this also means there is now a placeholder in every sequence above it:

0   3   6   9  12  15   B
  3   3   3   3   3   A
    0   0   0   0   0
You can then start filling in placeholders from the bottom up. A needs to be the result of increasing 3 (the value to its left) by 0 (the value below it); this means A must be 3:

0   3   6   9  12  15   B
  3   3   3   3   3   3
    0   0   0   0   0
Finally, you can fill in B, which needs to be the result of increasing 15 (the value to its left) by 3 (the value below it), or 18:

0   3   6   9  12  15  18
  3   3   3   3   3   3
    0   0   0   0   0
So, the next value of the first history is 18.

Finding all-zero differences for the second history requires an additional sequence:

1   3   6  10  15  21
  2   3   4   5   6
    1   1   1   1
      0   0   0
Then, following the same process as before, work out the next value in each sequence from the bottom up:

1   3   6  10  15  21  28
  2   3   4   5   6   7
    1   1   1   1   1
      0   0   0   0
So, the next value of the second history is 28.

The third history requires even more sequences, but its next value can be found the same way:

10  13  16  21  30  45  68
   3   3   5   9  15  23
     0   2   4   6   8
       2   2   2   2
         0   0   0
So, the next value of the third history is 68.

If you find the next value for each history in this example and add them
together, you get 114.

Analyze your OASIS report and extrapolate the next value for each history. What
is the sum of these extrapolated values?
*/

use anyhow::anyhow;
use nom::{
    bytes::complete::is_a,
    character::complete::{multispace0, newline, space0, space1},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<i32> {
    let readings = parse_input(input)?;

    let mut total = 0;
    for samples in readings {
        total += extrapolate(&samples);
    }
    Ok(total)
}

pub fn part2(input: &str) -> anyhow::Result<i32> {
    let readings = parse_input(input)?;

    let mut total = 0;
    for samples in readings {
        total += extrapolate_back(&samples);
    }
    Ok(total)
}

fn extrapolate(samples: &[i32]) -> i32 {
    if samples.iter().all(|&x| x == 0) {
        return 0;
    }
    let diffs: Vec<i32> = samples.windows(2).map(|w| w[1] - w[0]).collect();
    samples.last().unwrap() + extrapolate(&diffs)
}

fn extrapolate_back(samples: &[i32]) -> i32 {
    if samples.iter().all(|&x| x == 0) {
        return 0;
    }
    let diffs: Vec<i32> = samples.windows(2).map(|w| w[1] - w[0]).collect();
    samples.first().unwrap() - extrapolate_back(&diffs)
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    let (_, readings) = all_consuming(delimited(multispace0, readings_parser, multispace0))(input)
        .map_err(|err| anyhow!("could not parse {input}: {err}"))?;
    Ok(readings)
}

fn readings_parser(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline, samples_parser)(input)
}
fn samples_parser(input: &str) -> IResult<&str, Vec<i32>> {
    delimited(
        space0,
        separated_list1(space1, map_res(is_a("0123456789-"), str::parse)),
        space0,
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    ";

    #[test]
    fn parser_smoke_test() {
        assert_eq!(
            parse_input(SAMPLE_INPUT).unwrap(),
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
            ],
        );
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT).unwrap(), 114);
    }
    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day09.input").unwrap()).unwrap(),
            2075724761,
        );
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT).unwrap(), 2);
    }
    #[test]
    fn part2_real_input() {
        assert_eq!(
            part2(&std::fs::read_to_string("data/day09.input").unwrap()).unwrap(),
            1072,
        );
    }
}
