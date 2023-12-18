/*
--- Day 18: Lavaduct Lagoon ---
Thanks to your efforts, the machine parts factory is one of the first factories up and running since the lavafall came back. However, to catch up with the large backlog of parts requests, the factory will also need a large supply of lava for a while; the Elves have already started creating a large lagoon nearby for this purpose.

However, they aren't sure the lagoon will be big enough; they've asked you to take a look at the dig plan (your puzzle input). For example:

R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
The digger starts in a 1 meter cube hole in the ground. They then dig the specified number of meters up (U), down (D), left (L), or right (R), clearing full 1 meter cubes as they go. The directions are given as seen from above, so if "up" were north, then "right" would be east, and so on. Each trench is also listed with the color that the edge of the trench should be painted as an RGB hexadecimal color code.

When viewed from above, the above example dig plan would result in the following loop of trench (#) having been dug out from otherwise ground-level terrain (.):

#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######
At this point, the trench could contain 38 cubic meters of lava. However, this is just the edge of the lagoon; the next step is to dig out the interior so that it is one meter deep as well:

#######
#######
#######
..#####
..#####
#######
#####..
#######
.######
.######
Now, the lagoon can contain a much more respectable 62 cubic meters of lava. While the interior is dug out, the edges are also painted according to the color codes in the dig plan.

The Elves are concerned the lagoon won't be large enough; if they follow their dig plan, how many cubic meters of lava could it hold?
*/
use std::collections::HashSet;

use anyhow::{anyhow, bail};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, multispace0, multispace1, space1},
    combinator::{all_consuming, map_res, value},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

use crate::grid::{Direction, Position};

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let instructions = parse_input(input)?;

    let mut trench = HashSet::new();
    {
        let mut cur = Position(0, 0);
        for Instruction {
            direction,
            distance,
            ..
        } in instructions
        {
            for _ in 0..distance {
                cur = cur.step(direction);
                trench.insert(cur);
            }
        }
        if cur != Position(0, 0) {
            bail!("instructions did not form a closed loop, started at (0, 0), ended at {cur:?}");
        }
    }

    let (ilo, ihi, jlo, jhi) = (
        trench.iter().map(|p| p.0).min().unwrap() - 1,
        trench.iter().map(|p| p.0).max().unwrap() + 1,
        trench.iter().map(|p| p.1).min().unwrap() - 1,
        trench.iter().map(|p| p.1).max().unwrap() + 1,
    );
    let mut fill = HashSet::new();
    let mut stack = vec![Position(ilo, jlo)];
    while let Some(cur @ Position(i, j)) = stack.pop() {
        if trench.contains(&cur) || !fill.insert(cur) {
            continue;
        }
        if i > ilo {
            stack.push(Position(i - 1, j));
        }
        if i < ihi {
            stack.push(Position(i + 1, j));
        }
        if j > jlo {
            stack.push(Position(i, j - 1));
        }
        if j < jhi {
            stack.push(Position(i, j + 1));
        }
    }
    Ok((ihi.abs_diff(ilo) as usize + 1) * (jhi.abs_diff(jlo) as usize + 1) - fill.len())
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    distance: usize,
    _color: String,
}
fn parse_input(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let (_, instructions) =
        all_consuming(delimited(multispace0, instructions_parser, multispace0))(input)
            .map_err(|err| anyhow!("could not parse {input}: {err}"))?;
    Ok(instructions)
}
fn instructions_parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(multispace1, instruction_parser)(input)
}
fn instruction_parser(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = direction_parser(input)?;
    let (input, _) = space1(input)?;
    let (input, distance) = map_res(digit1, str::parse)(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = delimited(tag("(#"), hex_digit1, tag(")"))(input)?;
    Ok((
        input,
        Instruction {
            direction,
            distance,
            _color: color.to_owned(),
        },
    ))
}
fn direction_parser(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Left, tag("L")),
        value(Direction::Right, tag("R")),
        value(Direction::Up, tag("U")),
        value(Direction::Down, tag("D")),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    ";

    #[test]
    fn parser_smoke_test() {
        assert_eq!(parse_input(SAMPLE_INPUT).unwrap().len(), 14);
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT).unwrap(), 62);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day18.input").unwrap()).unwrap(),
            49061
        );
    }
}
