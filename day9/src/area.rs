use crate::tiles::RedTile;

pub struct Area<'a> {
    tile1: &'a RedTile,
    tile2: &'a RedTile,
    pub area: usize,
}

impl<'a> Area<'a> {
    pub fn new(tile1: &'a RedTile, tile2: &'a RedTile) -> Self {
        let area = calc_area(&tile1.position, &tile2.position);
        Area { tile1, tile2, area }
    }
}

fn calc_area(pos1: &(usize, usize), pos2: &(usize, usize)) -> usize {
    let x1 = pos1.0 as isize;
    let y1 = pos1.1 as isize;

    let x2 = pos2.0 as isize;
    let y2 = pos2.1 as isize;

    let x = (x1 - x2).abs() + 1;
    let y = (y1 - y2).abs() + 1;

    let area = x * y;
    area as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let pos1 = (2, 5);
        let pos2 = (11, 1);
        let area = calc_area(&pos2, &pos1);
        assert_eq!(50, area)
    }
}
