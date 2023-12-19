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

Your puzzle answer was 49061.

--- Part Two ---
The Elves were right to be concerned; the planned lagoon would be much too small.

After a few minutes, someone realizes what happened; someone swapped the color and instruction parameters when producing the dig plan. They don't have time to fix the bug; one of them asks if you can extract the correct instructions from the hexadecimal codes.

Each hexadecimal code is six hexadecimal digits long. The first five hexadecimal digits encode the distance in meters as a five-digit hexadecimal number. The last hexadecimal digit encodes the direction to dig: 0 means R, 1 means D, 2 means L, and 3 means U.

So, in the above example, the hexadecimal codes can be converted into the true instructions:

#70c710 = R 461937
#0dc571 = D 56407
#5713f0 = R 356671
#d2c081 = D 863240
#59c680 = R 367720
#411b91 = D 266681
#8ceee2 = L 577262
#caa173 = U 829975
#1b58a2 = L 112010
#caa171 = D 829975
#7807d2 = L 491645
#a77fa3 = U 686074
#015232 = L 5411
#7a21e3 = U 500254
Digging out this loop and its interior produces a lagoon that can hold an impressive 952408144115 cubic meters of lava.

Convert the hexadecimal color codes into the correct instructions; if the Elves follow this new dig plan, how many cubic meters of lava could the lagoon hold?

Your puzzle answer was 92556825427032.

*/
use crate::grid::Direction;
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

pub fn part1(input: &str) -> anyhow::Result<i64> {
    let instructions = parse_input(input)?;
    solve(
        instructions
            .iter()
            .map(|i| (i.direction, i.distance as i64)),
    )
}

pub fn part2(input: &str) -> anyhow::Result<i64> {
    let instructions = parse_input(input)?;
    solve(instructions.iter().map(|i| {
        let direction = match i.color[5] {
            b'0' => Direction::Right,
            b'1' => Direction::Down,
            b'2' => Direction::Left,
            b'3' => Direction::Up,
            _ => panic!("invalid color {:?}", i.color),
        };
        let distance =
            u32::from_str_radix(std::str::from_utf8(&i.color[..5]).unwrap(), 16).unwrap();
        (direction, distance as i64)
    }))
}

fn solve(moves: impl Iterator<Item = (Direction, i64)>) -> anyhow::Result<i64> {
    let mut area: i64 = 0;
    let mut perimeter: i64 = 0;
    let (mut x0, mut y0) = (0, 0);
    for (direction, distance) in moves {
        perimeter += distance;
        let (x1, y1) = match direction {
            Direction::Up => (x0, y0 + distance),
            Direction::Down => (x0, y0 - distance),
            Direction::Left => (x0 - distance, y0),
            Direction::Right => (x0 + distance, y0),
        };
        area += (y0 + y1) * (x1 - x0);
        (x0, y0) = (x1, y1);
    }
    if (x0, y0) != (0, 0) {
        bail!("instructions did not form a closed loop, started at (0, 0), ended at ({x0},{y0})");
    }
    Ok(area / 2 + perimeter / 2 + 1)
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    distance: u32,
    color: [u8; 6],
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
    let (input, color) = map_res(delimited(tag("(#"), hex_digit1, tag(")")), |s: &str| {
        s.as_bytes().to_vec().try_into()
    })(input)?;
    Ok((
        input,
        Instruction {
            direction,
            distance,
            color,
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

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT).unwrap(), 952408144115);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            part2(&std::fs::read_to_string("data/day18.input").unwrap()).unwrap(),
            92556825427032,
        );
    }
}
