use crate::grid::{Direction, Grid, Position};

/*
--- Day 16: The Floor Will Be Lava ---
With the beam of light completely focused somewhere, the reindeer leads you deeper still into the Lava Production Facility. At some point, you realize that the steel facility walls have been replaced with cave, and the doorways are just cave, and the floor is cave, and you're pretty sure this is actually just a giant cave.

Finally, as you approach what must be the heart of the mountain, you see a bright light in a cavern up ahead. There, you discover that the beam of light you so carefully focused is emerging from the cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite side.

Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing empty space (.), mirrors (/ and \), and splitters (| and -).

The contraption is aligned so that most of the beam bounces around the grid, but each tile on the grid converts some of the beam's light into heat to melt the rock in the cavern.

You note the layout of the contraption (your puzzle input). For example:

.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
The beam enters in the top-left corner from the left and heading to the right. Then, its behavior depends on what it encounters as it moves:

If the beam encounters empty space (.), it continues in the same direction.
If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would continue downward from the mirror's column.
If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter as if the splitter were empty space. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues upward from the splitter's column and one that continues downward from the splitter's column.
Beams do not interact with other beams; a tile can have many beams passing through it at the same time. A tile is energized if that tile has at least one beam pass through it, reflect in it, or split in it.

In the above example, here is how the beam of light bounces around the contraption:

>|<<<\....
|v-.\^....
.v...|->>>
.v...v^.|.
.v...v^...
.v...v^..\
.v../2\\..
<->-/vv|..
.|<<<2-|.\
.v//.|.v..
Beams are only shown on empty tiles; arrows indicate the direction of the beams. If a tile contains beams moving in multiple directions, the number of distinct directions is shown instead. Here is the same diagram but instead only showing whether a tile is energized (#) or not (.):

######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..
Ultimately, in this example, 46 tiles become energized.

The light isn't energizing enough tiles to produce lava; to debug the contraption, you need to start by analyzing the current situation. With the beam starting in the top-left heading right, how many tiles end up being energized?

*/
/*

--- Part Two ---
As you try to work out what might be wrong, the reindeer tugs on your shirt and leads you to a nearby control panel. There, a collection of buttons lets you align the contraption so that the beam enters from any edge tile and heading away from that edge. (You can choose either of two directions for the beam if it starts on a corner; for instance, if the beam starts in the bottom-right corner, it can start heading either left or upward.)

So, the beam could start on any tile in the top row (heading downward), any tile in the bottom row (heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost column (heading left). To produce lava, you need to find the configuration that energizes as many tiles as possible.

In the above example, this can be achieved by starting the beam in the fourth tile from the left in the top row:

.|<2<\....
|v-v\^....
.v.v.|->>>
.v.v.v^.|.
.v.v.v^...
.v.v.v^..\
.v.v/2\\..
<-2-/vv|..
.|<<<2-|.\
.v//.|.v..
Using this configuration, 51 tiles are energized:

.#####....
.#.#.#....
.#.#.#####
.#.#.##...
.#.#.##...
.#.#.##...
.#.#####..
########..
.#######..
.#...#.#..
Find the initial beam configuration that energizes the largest number of tiles; how many tiles are energized in that configuration?

*/

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
    collections::HashSet,
    fmt::{Debug, Write},
};
pub fn part1(input: &str) -> anyhow::Result<usize> {
    let grid = parse_input(input)?;
    Ok(count_energized(&grid, (Position(0, 0), Direction::Right)))
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = parse_input(input)?;
    let up = (0..grid.width())
        .map(|j| count_energized(&grid, (Position(grid.height() - 1, j), Direction::Up)));
    let down = (0..grid.width()).map(|j| count_energized(&grid, (Position(0, j), Direction::Down)));
    let right =
        (0..grid.height()).map(|i| count_energized(&grid, (Position(i, 0), Direction::Right)));
    let left = (0..grid.height())
        .map(|i| count_energized(&grid, (Position(i, grid.width() - 1), Direction::Left)));
    Ok(up.chain(down).chain(left).chain(right).max().unwrap_or(0))
}

fn count_energized(grid: &Grid<Cell>, enter: (Position, Direction)) -> usize {
    let mut vis: HashSet<(Position, Direction)> = HashSet::new();
    let mut stack: Vec<(Position, Direction)> = vec![enter];
    while let Some(e) = stack.pop() {
        let (pos @ Position(i, j), dir) = e;
        let Some(ch) = grid.get(i, j) else { continue };
        if !vis.insert(e) {
            continue;
        }
        let next_dirs = match ch {
            Cell::Ground => vec![dir],
            Cell::MirrorUp /* aka '/' */ => match dir {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            Cell::MirrorDown /* aka '\' */ => match dir {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            Cell::SplitHoriz => match dir {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => vec![dir],
            }
            Cell::SplitVert => match dir {
                Direction::Up | Direction::Down => vec![dir],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            }
        };
        for next_dir in next_dirs {
            stack.push((pos.step(next_dir), next_dir));
        }
    }
    // Count just the distinct positions we energized
    vis.into_iter()
        .map(|(pos, _dir)| pos)
        .collect::<HashSet<_>>()
        .len()
}

fn parse_input(input: &str) -> anyhow::Result<Grid<Cell>> {
    let input = input.trim();
    let (_, grid) = grid_parser(input).map_err(|err| anyhow!("could not parse {input}: {err}"))?;
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
            value(Cell::MirrorUp, tag("/")),
            value(Cell::MirrorDown, tag(r"\")),
            value(Cell::SplitVert, tag("|")),
            value(Cell::SplitHoriz, tag("-")),
        ))),
        space0,
    )(input)?;
    Ok((input, cells))
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    Ground,
    MirrorUp,
    MirrorDown,
    SplitVert,
    SplitHoriz,
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => f.write_char('.'),
            Self::MirrorUp => f.write_char('/'),
            Self::MirrorDown => f.write_char('\\'),
            Self::SplitVert => f.write_char('|'),
            Self::SplitHoriz => f.write_char('-'),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Dimensions;

    use super::*;

    const SAMPLE_INPUT: &str = r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    ";

    #[test]
    fn parser_smoke_test() {
        assert_eq!(
            parse_input(SAMPLE_INPUT).unwrap().size(),
            Dimensions {
                height: 10,
                width: 10
            }
        );
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT).unwrap(), 46);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day16.input").unwrap()).unwrap(),
            7236,
        );
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT).unwrap(), 51);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            part2(&std::fs::read_to_string("data/day16.input").unwrap()).unwrap(),
            7521,
        );
    }
}
