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
/*

--- Part Two ---
The crucibles of lava simply aren't large enough to provide an adequate supply of lava to the machine parts factory. Instead, the Elves are going to upgrade to ultra crucibles.

Ultra crucibles are even more difficult to steer than normal crucibles. Not only do they have trouble going in a straight line, but they also have trouble turning!

Once an ultra crucible starts moving in a direction, it needs to move a minimum of four blocks in that direction before it can turn (or even before it can stop at the end). However, it will eventually start to get wobbly: an ultra crucible can move a maximum of ten consecutive blocks without turning.

In the above example, an ultra crucible could follow this path to minimize heat loss:

2>>>>>>>>1323
32154535v5623
32552456v4254
34465858v5452
45466578v>>>>
143859879845v
445787698776v
363787797965v
465496798688v
456467998645v
122468686556v
254654888773v
432267465553v
In the above example, an ultra crucible would incur the minimum possible heat loss of 94.

Here's another example:

111111111111
999999999991
999999999991
999999999991
999999999991
Sadly, an ultra crucible would need to take an unfortunate path like this one:

1>>>>>>>1111
9999999v9991
9999999v9991
9999999v9991
9999999v>>>>
This route causes the ultra crucible to incur the minimum possible heat loss of 71.

Directing the ultra crucible from the lava pool to the machine parts factory, what is the least heat loss it can incur?
*/

use crate::grid::{Dimensions, Direction, Grid, Position};
use anyhow::anyhow;
use nom::{
    character::complete::{newline, one_of, space0},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::delimited,
    IResult,
};
use std::hash::Hash;

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let grid = parse_input(input)?;
    Ok(minimal_heat_loss_path(&grid, 1, 3))
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = parse_input(input)?;
    Ok(minimal_heat_loss_path(&grid, 4, 10))
}

fn minimal_heat_loss_path(grid: &Grid<usize>, min_steps: usize, max_steps: usize) -> usize {
    assert!(min_steps <= max_steps);
    let destination = Position(grid.height() - 1, grid.width() - 1);
    let mut queue: PriorityQueue = PriorityQueue::new(grid.size(), max_steps);
    queue.push(
        0,
        Location {
            position: Position(0, 0),
            direction: Direction::Right,
            gas: 0,
        },
    );
    queue.push(
        0,
        Location {
            position: Position(0, 0),
            direction: Direction::Down,
            gas: 0,
        },
    );

    while let Some(cur) = queue.pop() {
        let (heat_loss, location) = cur;
        let Location {
            position,
            direction,
            gas,
        } = location;
        let mut step = |new_direction: Direction| {
            let next = position.step(new_direction);
            if let Some(c) = grid.get(next.0, next.1) {
                let loc = Location {
                    position: next,
                    direction: new_direction,
                    gas: if new_direction == direction {
                        gas + 1
                    } else {
                        1
                    },
                };
                queue.push(heat_loss + c, loc);
            }
        };
        if gas < max_steps {
            step(direction);
        }
        if gas < min_steps {
            continue;
        }
        if position == destination {
            return heat_loss;
        }
        step(direction.clockwise());
        step(direction.counter_clockwise());
    }
    unreachable!("the grid must have a path through it")
}

struct PriorityQueue {
    queue: VecHeap<Location>,
    seen: LocationSet,
}
impl PriorityQueue {
    fn new(dimensions: Dimensions, max_steps: usize) -> Self {
        Self {
            queue: VecHeap::default(),
            seen: LocationSet::new(dimensions, max_steps + 1),
        }
    }
    fn push(&mut self, w: usize, t: Location) {
        if self.seen.insert(t) {
            self.queue.push(t, w);
        }
    }
    fn pop(&mut self) -> Option<(usize, Location)> {
        self.queue.pop()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)] // important that `heat_loss` comes first so that `Ord` weights it most
struct Entry {
    heat_loss: usize, // how much heat have we lost so far?
    location: Location,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location {
    position: Position,
    direction: Direction,
    gas: usize, // how many steps have we taken in this direction?
}
struct LocationSet {
    dimensions: Dimensions,
    max_steps: usize,
    bits: Vec<bool>,
}
impl LocationSet {
    fn new(dimensions: Dimensions, max_steps: usize) -> Self {
        Self {
            dimensions,
            max_steps,
            bits: vec![false; (dimensions.height * dimensions.width) as usize * max_steps * 4],
        }
    }
    fn insert(
        &mut self,
        Location {
            position,
            direction,
            gas,
        }: Location,
    ) -> bool {
        let idx: usize =
            ((position.0 * self.dimensions.width + position.1) as usize * self.max_steps + gas) * 4
                + usize::from(direction);
        let Some(cur) = self.bits.get_mut(idx) else {
            panic!("invalid location: {position:?}/{direction:?}/{gas}");
        };
        let prev = *cur;
        *cur = true;
        !prev
    }
}

struct VecHeap<T> {
    items: Vec<Vec<T>>,
    min_weight: usize,
}
impl<T> VecHeap<T> {
    fn push(&mut self, item: T, weight: usize) {
        while weight >= self.items.len() {
            self.items.push(Vec::new());
        }
        self.items[weight].push(item)
    }
    fn pop(&mut self) -> Option<(usize, T)> {
        for w in self.min_weight..self.items.len() {
            if let Some(item) = self.items[w].pop() {
                return Some((w, item));
            } else {
                self.min_weight = w + 1;
            }
        }
        None
    }
}
impl<T> Default for VecHeap<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            min_weight: 0,
        }
    }
}

fn parse_input(input: &str) -> anyhow::Result<Grid<usize>> {
    let input = input.trim();
    let (_, grid) = grid_parser(input).map_err(|err| anyhow!("could not parse {input}: {err}"))?;
    Ok(grid)
}
fn grid_parser(input: &str) -> IResult<&str, Grid<usize>> {
    map_res(separated_list1(newline, row_parser), Grid::new)(input)
}
fn row_parser(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, cells) = delimited(
        space0,
        many1(map_res(one_of("123456789"), |ch| {
            ch.to_digit(10)
                .map(|d| d as usize)
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
            771,
        );
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT).unwrap(), 94);

        assert_eq!(
            part2(
                "
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991
        "
            )
            .unwrap(),
            71
        );
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            parse_input(&std::fs::read_to_string("data/day17.input").unwrap())
                .unwrap()
                .size(),
            Dimensions {
                height: 141,
                width: 141
            },
        );
        assert_eq!(
            part2(&std::fs::read_to_string("data/day17.input").unwrap()).unwrap(),
            930,
        );
    }
}
