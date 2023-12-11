/*
--- Day 11: Cosmic Expansion ---
You continue following signs for "Hot Springs" and eventually come across an observatory. The Elf within turns out to be a researcher studying cosmic expansion using the giant telescope here.

He doesn't know anything about the missing machine parts; he's only visiting for this research project. However, he confirms that the hot springs are the next-closest area likely to have people; he'll even take you straight there once he's done with today's observation analysis.

Maybe you can help him with the analysis to speed things up?

The researcher has collected a bunch of data and compiled the data into a single giant image (your puzzle input). The image includes empty space (.) and galaxies (#). For example:

...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
The researcher is trying to figure out the sum of the lengths of the shortest path between every pair of galaxies. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.

Due to something involving gravitational effects, only some space expands. In fact, the result is that any rows or columns that contain no galaxies should all actually be twice as big.

In the above example, three columns and two rows contain no galaxies:

   v  v  v
 ...#......
 .......#..
 #.........
>..........<
 ......#...
 .#........
 .........#
>..........<
 .......#..
 #...#.....
   ^  ^  ^
These rows and columns need to be twice as big; the result of cosmic expansion therefore looks like this:

....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
Equipped with this expanded universe, the shortest path between every pair of galaxies can be found. It can help to assign every galaxy a unique number:

....1........
.........2...
3............
.............
.............
........4....
.5...........
............6
.............
.............
.........7...
8....9.......
In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the pair doesn't matter. For each pair, find any shortest path between the two galaxies using only steps that move up, down, left, or right exactly one . or # at a time. (The shortest path between two galaxies is allowed to pass through another galaxy.)

For example, here is one of the shortest paths between galaxies 5 and 9:

....1........
.........2...
3............
.............
.............
........4....
.5...........
.##.........6
..##.........
...##........
....##...7...
8....9.......
This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are some other example shortest path lengths:

Between galaxy 1 and galaxy 7: 15
Between galaxy 3 and galaxy 6: 17
Between galaxy 8 and galaxy 9: 5
In this example, after expanding the universe, the sum of the shortest path between all 36 pairs of galaxies is 374.

Expand the universe, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
*/

/*
--- Part Two ---
The galaxies are much older (and thus much farther apart) than the researcher initially estimated.

Now, instead of the expansion you did before, make each empty row or column one million times larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.

(In the example above, if each empty row or column were merely 10 times larger, the sum of the shortest paths between every pair of galaxies would be 1030. If each empty row or column were merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be 8410. However, your universe will need to expand far beyond these values.)

Starting with the same initial image, expand the universe according to these new rules, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
*/

use std::cmp;

use itertools::Itertools;

use crate::grid::Grid;

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let grid = parse_input(input)?;
    Ok(solve(&grid, 1))
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = parse_input(input)?;
    Ok(solve(&grid, 999_999))
}

fn solve(grid: &Grid<u8>, expansion: usize) -> usize {
    let stars: Vec<(i32, i32)> = grid
        .enumerate()
        .filter(|(_, &cell)| cell == b'#')
        .map(|(pos, _)| pos)
        .collect();
    let empty_rows: Vec<i32> = (0..grid.height())
        .filter(|&i| (0..grid.width()).all(|j| grid.get(i, j).copied() != Some(b'#')))
        .collect();
    let empty_cols: Vec<i32> = (0..grid.width())
        .filter(|&j| (0..grid.height()).all(|i| grid.get(i, j).copied() != Some(b'#')))
        .collect();

    let distances: Vec<usize> = stars
        .into_iter()
        .tuple_combinations()
        .map(|((i0, j0), (i1, j1))| {
            let (ilo, ihi) = (cmp::min(i0, i1), cmp::max(i0, i1));
            let (jlo, jhi) = (cmp::min(j0, j1), cmp::max(j0, j1));
            (ihi - ilo) as usize
                + (jhi - jlo) as usize
                + expansion * empty_rows.iter().filter(|&&i| ilo < i && i < ihi).count()
                + expansion * empty_cols.iter().filter(|&&j| jlo < j && j < jhi).count()
        })
        .collect();

    distances.into_iter().sum()
}

fn parse_input(input: &str) -> anyhow::Result<Grid<u8>> {
    Grid::new(
        input
            .trim()
            .lines()
            .map(|l| l.trim().as_bytes().to_vec())
            .collect(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    ";

    #[test]
    fn parser_smoke_test() {
        let grid = parse_input(SAMPLE_INPUT).unwrap();
        assert_eq!((grid.height(), grid.width()), (10, 10));
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT).unwrap(), 374);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(
            part1(&std::fs::read_to_string("data/day11.input").unwrap()).unwrap(),
            9177603,
        );
    }

    #[test]
    fn part2_sample_input() {
        let grid = parse_input(SAMPLE_INPUT).unwrap();
        assert_eq!(solve(&grid, 1), 374);
        assert_eq!(solve(&grid, 9), 1030);
        assert_eq!(solve(&grid, 99), 8410);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            part2(&std::fs::read_to_string("data/day11.input").unwrap()).unwrap(),
            632003913611,
        );
    }
}
