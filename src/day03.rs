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

use crate::grid::Grid;
use std::ops::Range;

pub fn part1(input: &str) -> anyhow::Result<u32> {
    let lines: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|l| l.trim().as_bytes().to_vec())
        .collect();
    let grid = Grid::new(lines)?;

    let mut ans = 0;
    for (i, row) in grid.rows().iter().enumerate() {
        let numbers = extract_numbers(row);
        for n in numbers {
            if has_adjacent_part(&grid, i as i32, n.span) {
                ans += n.value;
            }
        }
    }

    Ok(ans)
}

fn extract_numbers(line: &[u8]) -> Vec<Span<u32>> {
    let mut spans = Vec::new();
    let mut i = 0;
    while i < line.len() {
        if !line[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let mut j = i;
        let mut value = 0;
        while j < line.len() && line[j].is_ascii_digit() {
            value = 10 * value + (line[j] - b'0') as u32;
            j += 1;
        }
        spans.push(Span {
            span: i as i32..j as i32,
            value,
        });
        i = j;
    }
    spans
}

#[derive(Debug)]
struct Span<T> {
    span: Range<i32>,
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
}
