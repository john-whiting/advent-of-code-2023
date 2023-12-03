use crate::cell::Cells;

#[derive(Clone, PartialEq, Debug)]
pub struct Grid {
    pub rows: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        Self {
            rows: input
                .lines()
                .map(|line| line.trim().chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }

    pub fn cells(&self) -> Cells<'_> {
        Cells::new(self)
    }
}
