/*
--- Day 14: Parabolic Reflector Dish ---
You reach the place where all of the mirrors were pointing: a massive parabolic reflector dish attached to the side of another large mountain.

The dish is made up of many small mirrors, but while the mirrors themselves are roughly in the shape of a parabolic reflector dish, each individual mirror seems to be pointing in slightly the wrong direction. If the dish is meant to focus light, all it's doing right now is sending it in a vague direction.

This system must be what provides the energy for the lava! If you focus the reflector dish, maybe you can go where it's pointing and use the light to fix the lava production.

Upon closer inspection, the individual mirrors each appear to be connected via an elaborate system of ropes and pulleys to a large metal platform below the dish. The platform is covered in large rocks of various shapes. Depending on their position, the weight of the rocks deforms the platform, and the shape of the platform controls which ropes move and ultimately the focus of the dish.

In short: if you move the rocks, you can focus the dish. The platform even has a control panel on the side that lets you tilt it in one of four directions! The rounded rocks (O) will roll when the platform is tilted, while the cube-shaped rocks (#) will stay in place. You note the positions of all of the empty spaces (.) and rocks (your puzzle input). For example:

O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
Start by tilting the lever so all of the rocks will slide north as far as they will go:

OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
You notice that the support beams along the north side of the platform are damaged; to ensure the platform doesn't collapse, you should calculate the total load on the north support beams.

The amount of load caused by a single rounded rock (O) is equal to the number of rows from the rock to the south edge of the platform, including the row the rock is on. (Cube-shaped rocks (#) don't contribute to load.) So, the amount of load caused by each rock in each row is as follows:

OOOO.#.O.. 10
OO..#....#  9
OO..O##..O  8
O..#.OO...  7
........#.  6
..#....#.#  5
..O..#.O.O  4
..O.......  3
#....###..  2
#....#....  1
The total load is the sum of the load caused by all of the rounded rocks. In this example, the total load is 136.

Tilt the platform so that the rounded rocks all roll north. Afterward, what is the total load on the north support beams?
*/

use crate::grid::Grid;
use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space0},
    combinator::{map_res, value},
    multi::{many1, separated_list1},
    sequence::delimited,
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<i32> {
    let grid = parse_input(input)?;
    let mut total = 0;
    for j in 0..grid.width() {
        let mut next = 0;
        for i in 0..grid.height() {
            match grid.get(i, j).copied().unwrap() {
                Cell::Ground => {}
                Cell::Anchor => {
                    next = i + 1;
                }
                Cell::Rock => {
                    total += grid.height() - next;
                    next += 1;
                }
            }
        }
    }
    Ok(total)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Ground,
    Anchor,
    Rock,
}
fn parse_input(input: &str) -> anyhow::Result<Grid<Cell>> {
    let input = input.trim();
    let (_, grid) = grid_parser(input).map_err(|err| {
        let var_name = anyhow!("could not parse {input}: {err}");
        var_name
    })?;
    Ok(grid)
}
fn grid_parser(input: &str) -> IResult<&str, Grid<Cell>> {
    map_res(separated_list1(newline, row_parser), Grid::new)(input)
}
fn row_parser(input: &str) -> IResult<&str, Vec<Cell>> {
    let (input, cells) = delimited(
        space0,
        many1(alt((
            value(Cell::Ground, tag(".")),
            value(Cell::Anchor, tag("#")),
            value(Cell::Rock, tag("O")),
        ))),
        space0,
    )(input)?;
    Ok((input, cells))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    ";

    #[test]
    fn parser_smoke_test() {
        assert_eq!(parse_input(SAMPLE_INPUT).unwrap().width(), 10);
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT).unwrap(), 136);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day14.input").unwrap()).unwrap(),
            110090
        );
    }
}
