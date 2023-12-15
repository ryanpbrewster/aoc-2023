use crate::grid::Grid;
use anyhow::anyhow;
use itertools::iproduct;
use nom::{
    bytes::complete::is_a,
    character::complete::{multispace1, newline, space0},
    combinator::map_res,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/*
--- Day 13: Point of Incidence ---
With your help, the hot springs team locates an appropriate spring which launches you neatly and precisely up to the edge of Lava Island.

There's just one problem: you don't see any lava.

You do see a lot of ash and igneous rock; there are even what look like gray mountains scattered around. After a while, you make your way to a nearby cluster of mountains only to discover that the valley between them is completely full of large mirrors. Most of the mirrors seem to be aligned in a consistent way; perhaps you should head in that direction?

As you move through the valley of mirrors, you find that several of them have fallen from the large metal frames keeping them in place. The mirrors are extremely flat and shiny, and many of the fallen mirrors have lodged into the ash at strange angles. Because the terrain is all one color, it's hard to tell where it's safe to walk or where you're about to run into a mirror.

You note down the patterns of ash (.) and rocks (#) that you see as you walk (your puzzle input); perhaps by carefully analyzing these patterns, you can figure out where the mirrors are!

For example:

#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
To find the reflection in each pattern, you need to find a perfect reflection across either a horizontal line between two rows or across a vertical line between two columns.

In the first pattern, the reflection is across a vertical line between two columns; arrows on each of the two columns point at the line between the columns:

123456789
    ><
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
    ><
123456789
In this pattern, the line of reflection is the vertical line between columns 5 and 6. Because the vertical line is not perfectly in the middle of the pattern, part of the pattern (column 1) has nowhere to reflect onto and can be ignored; every other column has a reflected column within the pattern and must match exactly: column 2 matches column 9, column 3 matches 8, 4 matches 7, and 5 matches 6.

The second pattern reflects across a horizontal line instead:

1 #...##..# 1
2 #....#..# 2
3 ..##..### 3
4v#####.##.v4
5^#####.##.^5
6 ..##..### 6
7 #....#..# 7
This pattern reflects across the horizontal line between rows 4 and 5. Row 1 would reflect with a hypothetical row 8, but since that's not in the pattern, row 1 doesn't need to match anything. The remaining rows match: row 2 matches row 7, row 3 matches row 6, and row 4 matches row 5.

To summarize your pattern notes, add up the number of columns to the left of each vertical line of reflection; to that, also add 100 multiplied by the number of rows above each horizontal line of reflection. In the above example, the first pattern's vertical line has 5 columns to its left and the second pattern's horizontal line has 4 rows above it, a total of 405.

Find the line of reflection in each of the patterns in your notes. What number do you get after summarizing all of your notes?
*/

/*
--- Part Two ---
You resume walking through the valley of mirrors and - SMACK! - run directly into one. Hopefully nobody was watching, because that must have been pretty embarrassing.

Upon closer inspection, you discover that every mirror has exactly one smudge: exactly one . or # should be the opposite type.

In each pattern, you'll need to locate and fix the smudge that causes a different reflection line to be valid. (The old reflection line won't necessarily continue being valid after the smudge is fixed.)

Here's the above example again:

#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
The first pattern's smudge is in the top-left corner. If the top-left # were instead ., it would have a different, horizontal line of reflection:

1 ..##..##. 1
2 ..#.##.#. 2
3v##......#v3
4^##......#^4
5 ..#.##.#. 5
6 ..##..##. 6
7 #.#.##.#. 7
With the smudge in the top-left corner repaired, a new horizontal line of reflection between rows 3 and 4 now exists. Row 7 has no corresponding reflected row and can be ignored, but every other row matches exactly: row 1 matches row 6, row 2 matches row 5, and row 3 matches row 4.

In the second pattern, the smudge can be fixed by changing the fifth symbol on row 2 from . to #:

1v#...##..#v1
2^#...##..#^2
3 ..##..### 3
4 #####.##. 4
5 #####.##. 5
6 ..##..### 6
7 #....#..# 7
Now, the pattern has a different horizontal line of reflection between rows 1 and 2.

Summarize your notes as before, but instead use the new different reflection lines. In this example, the first pattern's new horizontal line has 3 rows above it and the second pattern's new horizontal line has 1 row above it, summarizing to the value 400.

In each pattern, fix the smudge and find the different line of reflection. What number do you get after summarizing the new reflection line in each pattern in your notes?
*/

pub fn part1(input: &str) -> anyhow::Result<i32> {
    let grids = parse_input(input)?;
    Ok(grids
        .iter()
        .map(|grid| find_symmetries(grid).into_iter().sum::<i32>())
        .sum())
}

pub fn part2(input: &str) -> anyhow::Result<i32> {
    let grids = parse_input(input)?;
    let mut total = 0;
    for mut grid in grids {
        let s0 = find_symmetries(&grid);
        for (i, j) in iproduct!(0..grid.height(), 0..grid.width()) {
            smudge(grid.get_mut(i, j).unwrap());
            let s1 = find_symmetries(&grid);
            if !s1.is_empty() && s1 != s0 {
                total += s1.into_iter().filter(|v| !s0.contains(v)).sum::<i32>();
                break;
            }
            smudge(grid.get_mut(i, j).unwrap());
        }
    }
    Ok(total)
}

fn smudge(c: &mut u8) {
    *c = if *c == b'#' { b'.' } else { b'#' };
}

fn find_symmetries(grid: &Grid<u8>) -> Vec<i32> {
    find_vertical_symmetry(grid)
        .into_iter()
        .chain(find_horizontal_symmetry(grid).into_iter().map(|v| 100 * v))
        .collect()
}
fn find_horizontal_symmetry(grid: &Grid<u8>) -> Vec<i32> {
    let digests: Vec<u64> = (0..grid.height())
        .map(|i| {
            let mut h = DefaultHasher::new();
            for j in 0..grid.width() {
                grid.get(i, j).hash(&mut h);
            }
            h.finish()
        })
        .collect();
    (1..grid.height())
        .filter(|&i| {
            (0..i)
                .rev()
                .zip(i..grid.height())
                .all(|(a, b)| digests[a as usize] == digests[b as usize])
        })
        .collect()
}
fn find_vertical_symmetry(grid: &Grid<u8>) -> Vec<i32> {
    let digests: Vec<u64> = (0..grid.width())
        .map(|j| {
            let mut h = DefaultHasher::new();
            for i in 0..grid.height() {
                grid.get(i, j).hash(&mut h);
            }
            h.finish()
        })
        .collect();
    (1..grid.width())
        .filter(|&j| {
            (0..j)
                .rev()
                .zip(j..grid.width())
                .all(|(a, b)| digests[a as usize] == digests[b as usize])
        })
        .collect()
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Grid<u8>>> {
    let input = input.trim();
    let (_, grids) =
        grids_parser(input).map_err(|err| anyhow!("could not parse {input}: {err}"))?;
    Ok(grids)
}
fn grids_parser(input: &str) -> IResult<&str, Vec<Grid<u8>>> {
    separated_list1(multispace1, grid_parser)(input)
}
fn grid_parser(input: &str) -> IResult<&str, Grid<u8>> {
    map_res(separated_list1(newline, row_parser), Grid::new)(input)
}
fn row_parser(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, cells) = delimited(space0, is_a("#."), space0)(input)?;
    Ok((input, cells.as_bytes().to_vec()))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    ";

    #[test]
    fn parser_smoke_test() {
        assert_eq!(parse_input(SAMPLE_INPUT).unwrap().len(), 2);
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT).unwrap(), 405);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day13.input").unwrap()).unwrap(),
            34772
        );
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT).unwrap(), 400);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            part2(&std::fs::read_to_string("data/day13.input").unwrap()).unwrap(),
            35554
        );
    }
}
