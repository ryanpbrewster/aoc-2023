/*
You and the Elf eventually reach a gondola lift station; he says the gondola
lift will take you up to the water source, but this is as far as he can bring
you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem:
they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of
surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working
right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine,
but nobody can figure out which one. If you can add up all the part numbers in
the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of
the engine. There are lots of numbers and symbols you don't really understand,
but apparently any number adjacent to a symbol, even diagonally, is a "part
number" and should be included in your sum. (Periods (.) do not count as a
symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not
adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number
is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of
the part numbers in the engine schematic?
*/

/*
--- Part Two ---
The engineer finds the missing part and installs it in the engine! As the engine
springs to life, you jump in the closest gondola, finally ready to ascend to the
water source.

You don't seem to be going very fast, though. Maybe something is still wrong?
Fortunately, the gondola has a phone labeled "help", so you pick it up and the
engineer answers.

Before you can explain the situation, she suggests that you look out the window.
There stands the engineer, holding a phone in one hand and waving with the
other. You're going so slowly that you haven't even left the station. You exit
the gondola.

The missing part wasn't the only issue - one of the gears in the engine is
wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its
gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so
that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, there are two gears. The first is in the top left; it has
part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the
lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear
because it is only adjacent to one part number.) Adding up all of the gear
ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?
*/

use crate::grid::Grid;
use std::{collections::HashSet, ops::Range};

pub fn part1(input: &str) -> anyhow::Result<u32> {
    let lines: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|l| l.trim().as_bytes().to_vec())
        .collect();
    let grid = Grid::new(lines)?;

    let numbers: HashSet<Span<u32>> = {
        let numgrid = extract_number_grid(&grid);
        numgrid
            .rows()
            .iter()
            .flat_map(|row| row.iter().flatten())
            .cloned()
            .collect()
    };

    let mut ans = 0;
    for span in numbers {
        if has_adjacent_part(&grid, span.i, span.jj) {
            ans += span.value;
        }
    }

    Ok(ans)
}

pub fn part2(input: &str) -> anyhow::Result<u32> {
    let lines: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|l| l.trim().as_bytes().to_vec())
        .collect();
    let grid = Grid::new(lines)?;
    let numgrid = extract_number_grid(&grid);

    let mut ans = 0;
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            if grid.get(i, j).copied() != Some(b'*') {
                continue;
            }
            let mut adjacent: HashSet<Span<u32>> = HashSet::new();
            for i in i - 1..=i + 1 {
                for j in j - 1..=j + 1 {
                    if let Some(Some(span)) = numgrid.get(i, j) {
                        adjacent.insert(span.clone());
                    }
                }
            }
            if adjacent.len() == 2 {
                ans += adjacent.into_iter().map(|s| s.value).product::<u32>();
            }
        }
    }

    Ok(ans)
}

// Take a schematic and construct a grid of numbers. At any given cell,
// the value will be Some(span) if the corresponding cell in the schematic
// is part of a number. The span will have enough info to uniquely identify
// that number.
// Example:
//   12.
//   .3.
// is a 2x3 schematic, and the corresponding "number grid" would have
//   AA.
//   .B.
// where A = {12, row 0, columns 0..=1}, B = {3, row 1, columns 1..=1}
fn extract_number_grid(grid: &Grid<u8>) -> Grid<Option<Span<u32>>> {
    let mut rows = Vec::new();
    for (i, row) in grid.rows().iter().enumerate() {
        let mut spans = Vec::new();
        let mut j0 = 0;
        while j0 < row.len() {
            if !row[j0].is_ascii_digit() {
                spans.push(None);
                j0 += 1;
                continue;
            }
            let mut j1 = j0;
            let mut value = 0;
            while j1 < row.len() && row[j1].is_ascii_digit() {
                value = 10 * value + (row[j1] - b'0') as u32;
                j1 += 1;
            }
            for _ in j0..j1 {
                spans.push(Some(Span {
                    i: i as i32,
                    jj: j0 as i32..j1 as i32,
                    value,
                }));
            }
            j0 = j1;
        }
        rows.push(spans);
    }
    Grid::new(rows).unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Span<T> {
    i: i32,
    jj: Range<i32>,
    value: T,
}

fn has_adjacent_part(grid: &Grid<u8>, i: i32, jj: Range<i32>) -> bool {
    (i - 1..=i + 1).any(|i| {
        (jj.start - 1..jj.end + 1).any(|j| is_part(grid.get(i, j).copied().unwrap_or(b'.')))
    })
}

fn is_part(ch: u8) -> bool {
    ch != b'.' && !ch.is_ascii_digit()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "
        .trim();
        assert_eq!(part1(input).unwrap(), 4361);
    }

    #[test]
    fn part1_real_input() {
        let input = std::fs::read_to_string("data/day03.input").unwrap();
        assert_eq!(part1(&input).unwrap(), 530849);
    }

    #[test]
    fn part2_sample_input() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "
        .trim();
        assert_eq!(part2(input).unwrap(), 467835);
    }

    #[test]
    fn part2_real_input() {
        let input = std::fs::read_to_string("data/day03.input").unwrap();
        assert_eq!(part2(&input).unwrap(), 84900879);
    }
}
