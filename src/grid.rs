use anyhow::bail;

#[derive(Clone)]
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
}

use std::fmt::{Debug, Write};
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
