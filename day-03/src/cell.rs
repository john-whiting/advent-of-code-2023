use crate::grid::Grid;
use crate::neighborhood::Neighborhood;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct PartNumber((usize, usize), pub usize);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell<'a> {
    pub grid: &'a Grid,
    pub row_idx: usize,
    pub col_idx: usize,
    pub value: char,
}

impl<'a> Cell<'a> {
    pub fn neighbors(&self) -> Neighborhood<'_> {
        Neighborhood::new(self)
    }

    pub fn part_number(&self) -> Option<PartNumber> {
        if !self.value.is_ascii_digit() {
            return None;
        }

        let max_col = self.grid.rows[0].len() - 1;
        let mut col = self.col_idx;

        while col > 0 && self.grid.rows[self.row_idx][col - 1].is_ascii_digit() {
            col -= 1;
        }

        let mut sum: usize = 0;

        while col <= max_col && self.grid.rows[self.row_idx][col].is_ascii_digit() {
            sum = (sum * 10) + (self.grid.rows[self.row_idx][col].to_digit(10).unwrap() as usize);
            col += 1;
        }

        Some(PartNumber((self.row_idx, col), sum))
    }
}

pub struct Cells<'a> {
    grid: &'a Grid,
    cur_row_idx: usize,
    cur_col_idx: usize,
}

impl<'a> Cells<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            cur_row_idx: 0,
            cur_col_idx: 0,
        }
    }
}

impl<'a> Iterator for Cells<'a> {
    type Item = Cell<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Get value and make the Cell
        let value = self.grid.rows[self.cur_row_idx][self.cur_col_idx];
        let cell = Cell {
            grid: self.grid,
            row_idx: self.cur_row_idx,
            col_idx: self.cur_col_idx,
            value,
        };

        // Update the indices accordingly
        self.cur_row_idx = (self.cur_row_idx + 1) % self.grid.rows.len();
        if self.cur_row_idx == 0 {
            self.cur_col_idx += 1;
        }

        // Ensure we are still within range
        if self.cur_col_idx >= self.grid.rows[0].len() {
            return None;
        }

        Some(cell)
    }
}
