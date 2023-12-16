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

    #[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
    struct Entry {
        pos: Position,
        dir: Direction,
    }
    let mut vis: HashSet<Entry> = HashSet::new();
    let mut stack: Vec<Entry> = vec![Entry {
        pos: Position(0, 0),
        dir: Direction::Right,
    }];
    while let Some(
        entry @ Entry {
            pos: pos @ Position(i, j),
            dir,
        },
    ) = stack.pop()
    {
        let Some(ch) = grid.get(i, j) else { continue };
        if !vis.insert(entry) {
            continue;
        }
        match ch {
            Cell::Ground => stack.push(Entry {
                pos: pos.step(dir),
                dir,
            }),
            Cell::MirrorUp => {
                // aka '/'
                let next_dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                stack.push(Entry {
                    pos: pos.step(next_dir),
                    dir: next_dir,
                });
            }
            Cell::MirrorDown => {
                // aka '\'
                let next_dir = match dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                stack.push(Entry {
                    pos: pos.step(next_dir),
                    dir: next_dir,
                });
            }
            Cell::SplitHoriz => {
                let next_dirs = match dir {
                    Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                    Direction::Left | Direction::Right => vec![dir],
                };
                stack.extend(next_dirs.into_iter().map(|next_dir| Entry {
                    pos: pos.step(next_dir),
                    dir: next_dir,
                }));
            }
            Cell::SplitVert => {
                let next_dirs = match dir {
                    Direction::Up | Direction::Down => vec![dir],
                    Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
                };
                stack.extend(next_dirs.into_iter().map(|next_dir| Entry {
                    pos: pos.step(next_dir),
                    dir: next_dir,
                }));
            }
        }
    }
    Ok(vis.into_iter().map(|e| e.pos).collect::<HashSet<_>>().len())
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
}
