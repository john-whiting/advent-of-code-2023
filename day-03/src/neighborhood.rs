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
    neighbor_indices: [(Option<usize>, Option<usize>); 8],
    count: usize,
}

impl<'a> Neighborhood<'a> {
    pub fn new(cell: &'a Cell) -> Self {
        Self {
            cell,
            neighbor_indices: Neighborhood::make_neighbor_indices(cell),
            count: 0,
        }
    }

    fn make_neighbor_indices(cell: &'a Cell) -> [(Option<usize>, Option<usize>); 8] {
        let max_row = cell.grid.rows.len() - 1;
        let max_col = cell.grid.rows[0].len() - 1;
        let left_idx = if cell.col_idx == 0 {
            None
        } else {
            Some(cell.col_idx - 1)
        };
        let right_idx = if cell.col_idx == max_col {
            None
        } else {
            Some(cell.col_idx + 1)
        };
        let up_idx = if cell.row_idx == 0 {
            None
        } else {
            Some(cell.row_idx - 1)
        };
        let down_idx = if cell.row_idx == max_row {
            None
        } else {
            Some(cell.row_idx + 1)
        };

        let cur_row = Some(cell.row_idx);
        let cur_col = Some(cell.col_idx);

        [
            (up_idx, left_idx),
            (up_idx, cur_col),
            (up_idx, right_idx),
            (cur_row, left_idx),
            (cur_row, right_idx),
            (down_idx, left_idx),
            (down_idx, cur_col),
            (down_idx, right_idx),
        ]
    }

    fn current_neighbor(&self) -> Option<Cell<'a>> {
        let rows = &self.cell.grid.rows;
        let (row_idx, col_idx) = self.neighbor_indices.get(self.count)?;

        if row_idx.is_some() && col_idx.is_some() {
            let row_idx = row_idx.unwrap();
            let col_idx = col_idx.unwrap();
            let value = rows[row_idx][col_idx];
            return Some(Cell {
                grid: self.cell.grid,
                value,
                row_idx,
                col_idx,
            });
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
