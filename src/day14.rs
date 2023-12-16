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

/*
--- Part Two ---
The parabolic reflector dish deforms, but not in a way that focuses the beam. To do that, you'll need to move the rocks to the edges of the platform. Fortunately, a button on the side of the control panel labeled "spin cycle" attempts to do just that!

Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then south, then east. After each tilt, the rounded rocks roll as far as they can before the platform tilts in the next direction. After one cycle, the platform will have finished rolling the rounded rocks in those four directions in that order.

Here's what happens in the example above after each of the first few cycles:

After 1 cycle:
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....

After 2 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O

After 3 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
This process should work if you leave it running long enough, but you're still worried about the north support beams. To make sure they'll survive for a while, you need to calculate the total load on the north support beams after 1000000000 cycles.

In the above example, after 1000000000 cycles, the total load on the north support beams is 64.

Run the spin cycle for 1000000000 cycles. Afterward, what is the total load on the north support beams?
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
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::{Debug, Write},
};

pub fn part1(input: &str) -> anyhow::Result<i32> {
    let mut grid = parse_input(input)?;
    tilt_up(&mut grid);
    let total = grid
        .enumerate()
        .filter(|(_pos, &cell)| cell == Cell::Rock)
        .map(|((i, _j), _cell)| grid.height() - i)
        .sum();
    Ok(total)
}

pub fn part2(input: &str) -> anyhow::Result<i32> {
    let mut grid = parse_input(input)?;

    let mut record: HashMap<Grid<Cell>, usize> = HashMap::new();
    let mut idx = 0;
    let cycle_length = loop {
        match record.entry(grid.clone()) {
            Entry::Occupied(prev) => break idx - prev.get(),
            Entry::Vacant(slot) => slot.insert(idx),
        };
        tilt_cycle(&mut grid);
        idx += 1;
    };

    let target = 1_000_000_000;
    idx += (target - idx) / cycle_length * cycle_length;

    while idx < target {
        tilt_cycle(&mut grid);
        idx += 1;
    }
    let total = grid
        .enumerate()
        .filter(|(_pos, &cell)| cell == Cell::Rock)
        .map(|((i, _j), _cell)| grid.height() - i)
        .sum();
    Ok(total)
}

fn tilt_cycle(grid: &mut Grid<Cell>) {
    tilt_up(grid);
    tilt_left(grid);
    tilt_down(grid);
    tilt_right(grid);
}
fn tilt_up(grid: &mut Grid<Cell>) {
    for j in 0..grid.width() {
        let mut next = 0;
        for i in 0..grid.height() {
            match grid[(i, j)] {
                Cell::Ground => {}
                Cell::Anchor => {
                    next = i + 1;
                }
                Cell::Rock => {
                    grid[(i, j)] = Cell::Ground;
                    grid[(next, j)] = Cell::Rock;
                    next += 1;
                }
            }
        }
    }
}

fn tilt_down(grid: &mut Grid<Cell>) {
    for j in 0..grid.width() {
        let mut next = grid.height() - 1;
        for i in (0..grid.height()).rev() {
            match grid[(i, j)] {
                Cell::Ground => {}
                Cell::Anchor => {
                    next = i - 1;
                }
                Cell::Rock => {
                    grid[(i, j)] = Cell::Ground;
                    grid[(next, j)] = Cell::Rock;
                    next -= 1;
                }
            }
        }
    }
}

fn tilt_left(grid: &mut Grid<Cell>) {
    for i in 0..grid.height() {
        let mut next = 0;
        for j in 0..grid.width() {
            match grid[(i, j)] {
                Cell::Ground => {}
                Cell::Anchor => {
                    next = j + 1;
                }
                Cell::Rock => {
                    grid[(i, j)] = Cell::Ground;
                    grid[(i, next)] = Cell::Rock;
                    next += 1;
                }
            }
        }
    }
}

fn tilt_right(grid: &mut Grid<Cell>) {
    for i in 0..grid.height() {
        let mut next = grid.width() - 1;
        for j in (0..grid.width()).rev() {
            match grid[(i, j)] {
                Cell::Ground => {}
                Cell::Anchor => {
                    next = j - 1;
                }
                Cell::Rock => {
                    grid[(i, j)] = Cell::Ground;
                    grid[(i, next)] = Cell::Rock;
                    next -= 1;
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    Ground,
    Anchor,
    Rock,
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => f.write_char('.'),
            Self::Anchor => f.write_char('#'),
            Self::Rock => f.write_char('O'),
        }
    }
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
            110090,
        );
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT).unwrap(), 64);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            part2(&std::fs::read_to_string("data/day14.input").unwrap()).unwrap(),
            95254,
        );
    }
}
