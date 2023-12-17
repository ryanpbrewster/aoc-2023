/*
--- Day 17: Clumsy Crucible ---
The lava starts flowing rapidly once the Lava Production Facility is operational. As you leave, the reindeer offers you a parachute, allowing you to quickly reach Gear Island.

As you descend, your bird's-eye view of Gear Island reveals why you had trouble finding anyone on your way up: half of Gear Island is empty, but the half below you is a giant factory city!

You land near the gradually-filling pool of lava at the base of your new lavafall. Lavaducts will eventually carry the lava throughout the city, but to make use of it immediately, Elves are loading it into large crucibles on wheels.

The crucibles are top-heavy and pushed by hand. Unfortunately, the crucibles become very difficult to steer at high speeds, and so it can be hard to go in a straight line for very long.

To get Desert Island the machine parts it needs as soon as possible, you'll need to find the best way to get the crucible from the lava pool to the machine parts factory. To do this, you need to minimize heat loss while choosing a route that doesn't require the crucible to go in a straight line for too long.

Fortunately, the Elves here have a map (your puzzle input) that uses traffic patterns, ambient temperature, and hundreds of other parameters to calculate exactly how much heat loss can be expected for a crucible entering any particular city block.

For example:

2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
Each city block is marked by a single digit that represents the amount of heat loss if the crucible enters that block. The starting point, the lava pool, is the top-left city block; the destination, the machine parts factory, is the bottom-right city block. (Because you already start in the top-left block, you don't incur that block's heat loss unless you leave that block and then return to it.)

Because it is difficult to keep the top-heavy crucible going in a straight line for very long, it can move at most three blocks in a single direction before it must turn 90 degrees left or right. The crucible also can't reverse direction; after entering each city block, it may only turn left, continue straight, or turn right.

One way to minimize heat loss is this path:

2>>34^>>>1323
32v>>>35v5623
32552456v>>54
3446585845v52
4546657867v>6
14385987984v4
44578769877v6
36378779796v>
465496798688v
456467998645v
12246868655<v
25465488877v5
43226746555v>
This path never moves more than three consecutive blocks in the same direction and incurs a heat loss of only 102.

Directing the crucible from the lava pool to the machine parts factory, but not moving more than three consecutive blocks in the same direction, what is the least heat loss it can incur?
*/

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use anyhow::anyhow;
use nom::{
    character::complete::{newline, one_of, space0},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::delimited,
    IResult,
};

use crate::grid::{Direction, Grid, Position};

pub fn part1(input: &str) -> anyhow::Result<u32> {
    let grid = parse_input(input)?;
    Ok(minimal_heat_loss_path(&grid))
}

fn minimal_heat_loss_path(grid: &Grid<u32>) -> u32 {
    let max_gas = 3;
    let destination = Position(grid.height() - 1, grid.width() - 1);
    let mut queue: PriorityQueue = PriorityQueue::default();
    queue.push(
        Location {
            position: Position(0, 0),
            direction: Direction::Right,
            gas: max_gas,
        },
        0,
    );

    while let Some(cur) = queue.pop() {
        let Entry {
            heat_loss,
            location:
                Location {
                    position,
                    direction,
                    gas,
                },
        } = cur;
        if position == destination {
            return heat_loss;
        }
        if gas > 0 {
            let dir = direction;
            let next @ Position(i, j) = position.step(dir);
            let loc = Location {
                position: next,
                direction: dir,
                gas: gas - 1,
            };
            if let Some(&cost) = grid.get(i, j) {
                queue.push(loc, heat_loss + cost);
            }
        }
        {
            let dir = direction.clockwise();
            let next @ Position(i, j) = position.step(dir);
            let loc = Location {
                position: next,
                direction: dir,
                gas: max_gas - 1,
            };
            if let Some(&cost) = grid.get(i, j) {
                queue.push(loc, heat_loss + cost);
            }
        }
        {
            let dir = direction.counter_clockwise();
            let next @ Position(i, j) = position.step(dir);
            let loc = Location {
                position: next,
                direction: dir,
                gas: max_gas - 1,
            };
            if let Some(&cost) = grid.get(i, j) {
                queue.push(loc, heat_loss + cost);
            }
        }
    }
    unreachable!("the grid must have a path through it")
}

#[derive(Default)]
struct PriorityQueue {
    queue: BinaryHeap<Reverse<Entry>>,
    seen: HashSet<Location>,
}
impl PriorityQueue {
    fn push(&mut self, location: Location, heat_loss: u32) {
        if self.seen.insert(location) {
            self.queue.push(Reverse(Entry {
                heat_loss,
                location,
            }));
        }
    }
    fn pop(&mut self) -> Option<Entry> {
        self.queue.pop().map(|Reverse(cur)| cur)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)] // important that `heat_loss` comes first so that `Ord` weights it most
struct Entry {
    heat_loss: u32, // how much heat have we lost so far?
    location: Location,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location {
    position: Position,
    direction: Direction,
    gas: u32, // how many steps we can take in this direction before we must turn
}

fn parse_input(input: &str) -> anyhow::Result<Grid<u32>> {
    let input = input.trim();
    let (_, grid) = grid_parser(input).map_err(|err| anyhow!("could not parse {input}: {err}"))?;
    Ok(grid)
}
fn grid_parser(input: &str) -> IResult<&str, Grid<u32>> {
    map_res(separated_list1(newline, row_parser), Grid::new)(input)
}
fn row_parser(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, cells) = delimited(
        space0,
        many1(map_res(one_of("123456789"), |ch| {
            ch.to_digit(10)
                .ok_or_else(|| anyhow!("could not parse decimal number from {ch}"))
        })),
        space0,
    )(input)?;
    Ok((input, cells))
}

#[cfg(test)]
mod test {
    use crate::grid::Dimensions;

    use super::*;

    const SAMPLE_INPUT: &str = "
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    ";

    #[test]
    fn parser_smoke_test() {
        assert_eq!(
            parse_input(SAMPLE_INPUT).unwrap().size(),
            Dimensions {
                height: 13,
                width: 13
            }
        );
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT).unwrap(), 102);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day17.input").unwrap()).unwrap(),
            771
        );
    }
}
