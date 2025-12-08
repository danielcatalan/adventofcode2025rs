use std::io::BufRead;

use crate::grid;

pub struct Grid {
    map: Vec<Vec<Option<RollsOfPaper>>>,
    rows_len: usize,
    columns_len: usize,
}

struct RollsOfPaper {}

pub fn parse_grid<R: BufRead>(reader: R) -> Grid {
    let x = reader.lines().map(|s| parse_row(s.unwrap())).collect();
    Grid::new(x)
}

fn parse_row(s: String) -> Vec<Option<RollsOfPaper>> {
    s.as_bytes()
        .iter()
        .map(|b| {
            if *b == b'@' {
                Some(RollsOfPaper {})
            } else {
                None
            }
        })
        .collect()
}

impl Grid {
    fn new(map: Vec<Vec<Option<RollsOfPaper>>>) -> Grid {
        let rows_len = map.len();
        let columns_len = map[0].len();
        Grid {
            map,
            rows_len,
            columns_len,
        }
    }

    pub fn total_accessible_rolls(&self) -> usize {
        let rows = self.rows_len;
        let columns = self.columns_len;
        let mut total_accessible = 0;
        for r in 0..rows {
            for c in 0..columns {
                if self.is_roll_accessible(r as i32, c as i32) {
                    total_accessible += 1;
                }
            }
        }
        total_accessible
    }

    fn is_roll_accessible(&self, r: i32, c: i32) -> bool {
        if !self.is_roll(r, c) {
            return false;
        }

        let checks = [
            (r - 1, c - 1),
            (r - 1, c),
            (r - 1, c + 1),
            (r, c - 1),
            (r, c + 1),
            (r + 1, c - 1),
            (r + 1, c),
            (r + 1, c + 1),
        ];
        let mut roll_counter = 0;
        for (r, c) in checks {
            if self.is_roll(r, c) {
                roll_counter += 1;
            }
            if roll_counter >= 4 {
                return false;
            }
        }
        return true;
    }

    fn is_roll(&self, r: i32, c: i32) -> bool {
        if (r < 0) || (c < 0) {
            return false;
        }

        if (r as usize >= self.rows_len) || (c as usize >= self.columns_len) {
            return false;
        }

        if self.map[r as usize][c as usize].is_some() {
            return true;
        }
        false
    }
}
