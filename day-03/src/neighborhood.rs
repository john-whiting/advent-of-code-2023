use crate::cell::Cell;

/// An iterator that goes through the surrounding cells of a cell
///
/// The following is the order of exploration with C being the current cell
/// 0 1 2
/// 3 C 4
/// 5 6 7
#[derive(Clone, PartialEq, Debug)]
pub struct Neighborhood<'a> {
    cell: &'a Cell<'a>,
    left_idx: Option<usize>,
    right_idx: Option<usize>,
    up_idx: Option<usize>,
    down_idx: Option<usize>,
    count: u8,
}

impl<'a> Neighborhood<'a> {
    pub fn new(cell: &'a Cell) -> Self {
        let max_row = cell.grid.rows.len() - 1;
        let max_col = cell.grid.rows[0].len() - 1;
        Self {
            cell,
            left_idx: if cell.col_idx == 0 {
                None
            } else {
                Some(cell.col_idx - 1)
            },
            right_idx: if cell.col_idx == max_col {
                None
            } else {
                Some(cell.col_idx + 1)
            },
            up_idx: if cell.row_idx == 0 {
                None
            } else {
                Some(cell.row_idx - 1)
            },
            down_idx: if cell.row_idx == max_row {
                None
            } else {
                Some(cell.row_idx + 1)
            },
            count: 0,
        }
    }

    fn current_neighbor(&self) -> Option<Cell<'a>> {
        let rows = &self.cell.grid.rows;
        match self.count {
            0 => {
                if self.left_idx.is_some() && self.up_idx.is_some() {
                    let row_idx = self.up_idx.unwrap();
                    let col_idx = self.left_idx.unwrap();
                    let value = rows[row_idx][col_idx];
                    return Some(Cell {
                        grid: self.cell.grid,
                        value,
                        row_idx,
                        col_idx,
                    });
                }
            }
            1 => {
                if let Some(up) = self.up_idx {
                    let row_idx = up;
                    let col_idx = self.cell.col_idx;
                    let value = rows[row_idx][col_idx];
                    return Some(Cell {
                        grid: self.cell.grid,
                        value,
                        row_idx,
                        col_idx,
                    });
                }
            }
            2 => {
                if self.right_idx.is_some() && self.up_idx.is_some() {
                    let row_idx = self.up_idx.unwrap();
                    let col_idx = self.right_idx.unwrap();
                    let value = rows[row_idx][col_idx];
                    return Some(Cell {
                        grid: self.cell.grid,
                        value,
                        row_idx,
                        col_idx,
                    });
                }
            }
            3 => {
                if let Some(left) = self.left_idx {
                    let row_idx = self.cell.row_idx;
                    let col_idx = left;
                    let value = rows[row_idx][col_idx];
                    return Some(Cell {
                        grid: self.cell.grid,
                        value,
                        row_idx,
                        col_idx,
                    });
                }
            }
            4 => {
                if let Some(right) = self.right_idx {
                    let row_idx = self.cell.row_idx;
                    let col_idx = right;
                    let value = rows[row_idx][col_idx];
                    return Some(Cell {
                        grid: self.cell.grid,
                        value,
                        row_idx,
                        col_idx,
                    });
                }
            }
            5 => {
                if self.left_idx.is_some() && self.down_idx.is_some() {
                    let row_idx = self.down_idx.unwrap();
                    let col_idx = self.left_idx.unwrap();
                    let value = rows[row_idx][col_idx];
                    return Some(Cell {
                        grid: self.cell.grid,
                        value,
                        row_idx,
                        col_idx,
                    });
                }
            }
            6 => {
                if let Some(down) = self.down_idx {
                    let row_idx = down;
                    let col_idx = self.cell.col_idx;
                    let value = rows[row_idx][col_idx];
                    return Some(Cell {
                        grid: self.cell.grid,
                        value,
                        row_idx,
                        col_idx,
                    });
                }
            }
            7 => {
                if self.right_idx.is_some() && self.down_idx.is_some() {
                    let row_idx = self.down_idx.unwrap();
                    let col_idx = self.right_idx.unwrap();
                    let value = rows[row_idx][col_idx];
                    return Some(Cell {
                        grid: self.cell.grid,
                        value,
                        row_idx,
                        col_idx,
                    });
                }
            }
            _ => (),
        }

        None
    }
}

impl<'a> Iterator for Neighborhood<'a> {
    type Item = Cell<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_value: Option<Self::Item> = None;

        while next_value.is_none() && self.count < 8 {
            next_value = self.current_neighbor();
            self.count += 1;
        }

        next_value
    }
}
