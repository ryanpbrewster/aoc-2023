use anyhow::bail;

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

    pub fn get(&self, i: i32, j: i32) -> Option<&T> {
        if i < 0 || i >= self.height || j < 0 || j >= self.width {
            return None;
        }
        Some(&self.rows[i as usize][j as usize])
    }
    pub fn row(&self, i: i32) -> Option<&[T]> {
        if i < 0 || i >= self.height {
            return None;
        }
        Some(&self.rows[i as usize])
    }
}
