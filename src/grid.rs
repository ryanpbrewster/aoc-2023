use anyhow::bail;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    height: i32,
    width: i32,
    rows: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(rows: Vec<Vec<T>>) -> anyhow::Result<Grid<T>> {
        let height = rows.len();
        if height == 0 {
            bail!("empty grid");
        }
        let width = rows[0].len();
        if rows.iter().any(|row| row.len() != width) {
            bail!("uneven rows");
        }
        Ok(Grid {
            height: height as i32,
            width: width as i32,
            rows,
        })
    }

    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn size(&self) -> Dimensions {
        Dimensions {
            height: self.height,
            width: self.width,
        }
    }
    pub fn rows(&self) -> &Vec<Vec<T>> {
        &self.rows
    }

    pub fn contains(&self, i: i32, j: i32) -> bool {
        0 <= i && i < self.height && 0 <= j && j < self.width
    }
    pub fn get(&self, i: i32, j: i32) -> Option<&T> {
        if self.contains(i, j) {
            Some(&self.rows[i as usize][j as usize])
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, i: i32, j: i32) -> Option<&mut T> {
        if self.contains(i, j) {
            Some(&mut self.rows[i as usize][j as usize])
        } else {
            None
        }
    }
    pub fn row(&self, i: i32) -> Option<&[T]> {
        if i < 0 || i >= self.height {
            return None;
        }
        Some(&self.rows[i as usize])
    }

    pub fn enumerate(&self) -> impl Iterator<Item = ((i32, i32), &T)> {
        itertools::iproduct!(0..self.height, 0..self.width)
            .map(|(i, j)| ((i, j), &self.rows[i as usize][j as usize]))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Dimensions {
    pub height: i32,
    pub width: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position(pub i32, pub i32);
impl Position {
    pub fn step(self, dir: Direction) -> Self {
        let Position(i, j) = self;
        match dir {
            Direction::Up => Position(i - 1, j),
            Direction::Down => Position(i + 1, j),
            Direction::Left => Position(i, j - 1),
            Direction::Right => Position(i, j + 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    fn index(&self, (i, j): (i32, i32)) -> &T {
        &self.rows[i as usize][j as usize]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    fn index_mut(&mut self, (i, j): (i32, i32)) -> &mut T {
        &mut self.rows[i as usize][j as usize]
    }
}

use std::{
    fmt::{Debug, Write},
    ops::{Index, IndexMut},
};
impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for cell in row {
                cell.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
