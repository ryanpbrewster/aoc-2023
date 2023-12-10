use std::{collections::HashSet, fmt::Write};

use crate::grid::Grid;
use anyhow::{anyhow, bail};

/*
--- Day 10: Pipe Maze ---
You use the hang glider to ride the hot air from Desert Island all the way up to
the floating metal island. This island is surprisingly cold and there definitely
aren't any thermals to glide on, so you leave your hang glider behind.

You wander around for a while, but you don't find any people or animals.
However, you do occasionally find signposts labeled "Hot Springs" pointing in a
seemingly consistent direction; maybe you can find someone at the hot springs
and ask them where the desert-machine parts are made.

The landscape here is alien; even the flowers and trees are made of metal. As
you stop to admire some metal grass, you notice something metallic scurry away
in your peripheral vision and jump into a big pipe! It didn't look like any
animal you've ever seen; if you want a better look, you'll need to get ahead of
it.

Scanning the area, you discover that the entire field you're standing on is
densely packed with pipes; it was hard to tell at first because they're the same
metallic silver color as the "ground". You make a quick sketch of all of the
surface pipes you can see (your puzzle input).

The pipes are arranged in a two-dimensional grid of tiles:

| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

Based on the acoustics of the animal's scurrying, you're confident the pipe that
contains the animal is one large, continuous loop.

For example, here is a square loop of pipe:

.....
.F-7.
.|.|.
.L-J.
.....
If the animal had entered this loop in the northwest corner, the sketch would instead look like this:

.....
.S-7.
.|.|.
.L-J.
.....

In the above diagram, the S tile is still a 90-degree F bend: you can tell
because of how the adjacent pipes connect to it.

Unfortunately, there are also many pipes that aren't connected to the loop! This
sketch shows the same loop as above:

-L|F7
7S-7|
L|7||
-L-J|
L|-JF

In the above diagram, you can still figure out which pipes form the main loop:
they're the ones connected to S, pipes those pipes connect to, pipes those pipes
connect to, and so on. Every pipe in the main loop connects to its two neighbors
(including S, which will have exactly two pipes connecting to it, and which is
assumed to connect back to those two pipes).

Here is a sketch that contains a slightly more complex main loop:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...

Here's the same example sketch with the extra, non-main-loop pipe tiles also
shown:

7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ

If you want to get out ahead of the animal, you should find the tile in the loop
that is farthest from the starting position. Because the animal is in the pipe,
it doesn't make sense to measure this by direct distance. Instead, you need to
find the tile that would take the longest number of steps along the loop to
reach from the starting point - regardless of which way around the loop the
animal went.

In the first example with the square loop:

.....
.S-7.
.|.|.
.L-J.
.....

You can count the distance each tile in the loop is from the starting point like
this:

.....
.012.
.1.3.
.234.
.....
In this example, the farthest point from the start is 4 steps away.

Here's the more complex loop again:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...
Here are the distances for each tile on that loop:

..45.
.236.
01.78
14567
23...
Find the single giant loop starting at S. How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?
*/

/*
You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is within the area enclosed by the loop?

To determine whether it's even worth taking the time to search for such a nest, you should calculate how many tiles are contained within the loop. For example:

...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
The above loop encloses merely four tiles - the two pairs of . in the southwest and southeast (marked I below). The middle . tiles (marked O below) are not in the loop. Here is the same loop again with those regions marked:

...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....
In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed! Here, I is still within the loop and O is still outside the loop:

..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
In both of the above examples, 4 tiles are enclosed by the loop.

Here's a larger example:

.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
The above sketch has many random bits of ground, some of which are in the loop (I) and some of which are outside it (O):

OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO
In this larger example, 8 tiles are enclosed by the loop.

Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's another example with many bits of junk pipe lying around that aren't connected to the main loop at all:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
Here are just the tiles that are enclosed by the loop marked with I:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
In this last example, 10 tiles are enclosed by the loop.

Figure out whether you have time to search for the nest by calculating the area within the loop. How many tiles are enclosed by the loop?
*/

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let grid = parse_input(input)?;
    let Some((i, j)) = itertools::iproduct!(0..grid.height(), 0..grid.width())
        .find(|&(i, j)| grid.get(i, j) == Some(&Tile::Animal))
    else {
        bail!("no animal found in grid");
    };
    for dir in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        if let Ok(path) = extract_loop(&grid, (i, j), dir) {
            return Ok(path.len() / 2);
        }
    }
    Err(anyhow!("no closed loop"))
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = parse_input(input)?;
    let Some((i, j)) = itertools::iproduct!(0..grid.height(), 0..grid.width())
        .find(|&(i, j)| grid.get(i, j) == Some(&Tile::Animal))
    else {
        bail!("no animal found in grid");
    };
    let Some(path) = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .into_iter()
    .find_map(|dir| extract_loop(&grid, (i, j), dir).ok()) else {
        bail!("no closed loop")
    };

    let pathset: HashSet<(i32, i32)> = path.iter().copied().collect();
    let mut vis = HashSet::new();
    let mut stack = vec![((0, 0), Direction::North)];
    while let Some(((i, j), bias)) = stack.pop() {
        if vis.contains(&(i, j)) {
            continue;
        }
        let Some(mut tile) = grid.get(i, j).copied() else {
            continue;
        };
        vis.insert((i, j));
        if !pathset.contains(&(i, j)) {
            tile = Tile::Ground;
        };
        match (tile, bias) {
            (Tile::Ground, _) => {
                stack.push(((i + 1, j), Direction::North));
                stack.push(((i - 1, j), Direction::South));
                stack.push(((i, j + 1), Direction::West));
                stack.push(((i, j - 1), Direction::East));
            }
            (Tile::Animal, _) => {}
            (Tile::Connector(Pipe::NS), dir) => {
                stack.push(((i + 1, j), dir));
                stack.push(((i - 1, j), dir));
                stack.push((dir.step((i, j)), dir.rev()));
            }
            (Tile::Connector(Pipe::EW), dir) => {
                stack.push(((i, j + 1), dir));
                stack.push(((i, j - 1), dir));
                stack.push((dir.step((i, j)), dir.rev()));
            }
            (Tile::Connector(Pipe::NE), Direction::North | Direction::East) => {
                stack.push(((i, j + 1), Direction::North));
                stack.push(((i - 1, j), Direction::East));
            }
            (Tile::Connector(Pipe::NE), Direction::South | Direction::West) => {
                stack.push(((i, j + 1), Direction::South));
                stack.push(((i - 1, j), Direction::West));
                stack.push(((i, j - 1), Direction::East));
                stack.push(((i + 1, j), Direction::North));
            }
            (Tile::Connector(Pipe::ES), Direction::South | Direction::East) => {
                stack.push(((i, j + 1), Direction::South));
                stack.push(((i + 1, j), Direction::East));
            }
            (Tile::Connector(Pipe::ES), Direction::North | Direction::West) => {
                stack.push(((i, j + 1), Direction::North));
                stack.push(((i - 1, j), Direction::South));
                stack.push(((i, j - 1), Direction::East));
                stack.push(((i + 1, j), Direction::West));
            }
            (Tile::Connector(Pipe::SW), Direction::North | Direction::East) => {
                stack.push(((i, j + 1), Direction::West));
                stack.push(((i - 1, j), Direction::South));
                stack.push(((i, j - 1), Direction::North));
                stack.push(((i + 1, j), Direction::East));
            }
            (Tile::Connector(Pipe::SW), Direction::South | Direction::West) => {
                stack.push(((i, j - 1), Direction::South));
                stack.push(((i + 1, j), Direction::West));
            }
            (Tile::Connector(Pipe::NW), Direction::North | Direction::West) => {
                stack.push(((i - 1, j), Direction::West));
                stack.push(((i, j - 1), Direction::North));
            }
            (Tile::Connector(Pipe::NW), Direction::South | Direction::East) => {
                stack.push(((i, j + 1), Direction::West));
                stack.push(((i - 1, j), Direction::East));
                stack.push(((i, j - 1), Direction::South));
                stack.push(((i + 1, j), Direction::North));
            }
        }
    }
    Ok(grid.height() as usize * grid.width() as usize - vis.len())
}

fn extract_loop(
    grid: &Grid<Tile>,
    start: (i32, i32),
    mut dir: Direction,
) -> anyhow::Result<Vec<(i32, i32)>> {
    let mut path = Vec::new();
    let (mut i, mut j) = start;
    loop {
        path.push((i, j));
        let (ii, jj) = dir.step((i, j));
        if path.contains(&(ii, jj)) {
            return Ok(path);
        }
        let Some(&Tile::Connector(pipe)) = grid.get(ii, jj) else {
            bail!("bottomed out at ({ii}, {jj}) going {dir:?}");
        };
        dir = match (dir, pipe) {
            (Direction::South, Pipe::NE) => Direction::East,
            (Direction::South, Pipe::NS) => Direction::South,
            (Direction::South, Pipe::NW) => Direction::West,
            (Direction::East, Pipe::NW) => Direction::North,
            (Direction::East, Pipe::EW) => Direction::East,
            (Direction::East, Pipe::SW) => Direction::South,
            (Direction::North, Pipe::ES) => Direction::East,
            (Direction::North, Pipe::NS) => Direction::North,
            (Direction::North, Pipe::SW) => Direction::West,
            (Direction::West, Pipe::NE) => Direction::North,
            (Direction::West, Pipe::ES) => Direction::South,
            (Direction::West, Pipe::EW) => Direction::West,
            _ => bail!("bad pipe {dir:?} --> {pipe:?}"),
        };
        (i, j) = (ii, jj);
    }
}

fn parse_input(input: &str) -> anyhow::Result<Grid<Tile>> {
    let rows: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(str::trim)
        .map(|line| {
            line.as_bytes()
                .iter()
                .copied()
                .map(Tile::try_from)
                .collect::<anyhow::Result<_>>()
        })
        .collect::<anyhow::Result<_>>()?;
    Grid::new(rows)
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Connector(Pipe),
    Ground,
    Animal,
}
impl TryFrom<u8> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> anyhow::Result<Self> {
        match value {
            b'|' => Ok(Self::Connector(Pipe::NS)),
            b'-' => Ok(Self::Connector(Pipe::EW)),
            b'L' => Ok(Self::Connector(Pipe::NE)),
            b'J' => Ok(Self::Connector(Pipe::NW)),
            b'7' => Ok(Self::Connector(Pipe::SW)),
            b'F' => Ok(Self::Connector(Pipe::ES)),
            b'.' => Ok(Self::Ground),
            b'S' => Ok(Self::Animal),
            _ => Err(anyhow!("unrecognized tile: {value}")),
        }
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Self::Connector(Pipe::NS) => '|',
            Self::Connector(Pipe::EW) => '-',
            Self::Connector(Pipe::NE) => 'L',
            Self::Connector(Pipe::NW) => 'J',
            Self::Connector(Pipe::SW) => '7',
            Self::Connector(Pipe::ES) => 'F',
            Tile::Ground => '.',
            Tile::Animal => 'S',
        };
        f.write_char(ch)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn rev(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    fn step(&self, (i, j): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (i - 1, j),
            Direction::South => (i + 1, j),
            Direction::East => (i, j + 1),
            Direction::West => (i, j - 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    NE,
    NS,
    NW,
    ES,
    EW,
    SW,
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT_1: &str = "
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    ";

    const SAMPLE_INPUT_2: &str = "
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    ";

    #[test]
    fn parser_smoke_test() {
        parse_input(SAMPLE_INPUT_1).unwrap();
        parse_input(SAMPLE_INPUT_2).unwrap();
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT_1).unwrap(), 4);
        assert_eq!(part1(SAMPLE_INPUT_2).unwrap(), 8);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day10.input").unwrap()).unwrap(),
            6812,
        );
    }

    const SAMPLE_INPUT_3: &str = "
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    ";
    const SAMPLE_INPUT_4: &str = "
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    ";

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT_1).unwrap(), 1);
        assert_eq!(part2(SAMPLE_INPUT_3).unwrap(), 4);
        assert_eq!(part2(SAMPLE_INPUT_4).unwrap(), 10);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            part2(&std::fs::read_to_string("data/day10.input").unwrap()).unwrap(),
            527,
        );
    }
}
