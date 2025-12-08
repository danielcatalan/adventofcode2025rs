use std::collections::HashSet;
use std::io::BufRead;
use std::ops::RangeInclusive;

pub struct Fridge {
    fresh_id_ranges: Vec<RangeInclusive<usize>>,
    ids: Vec<usize>,
}
impl Fridge {
    pub fn total_fresh_ids(&self) -> usize {
        let mut total_fresh = HashSet::new();

        for id in &self.ids {
            if total_fresh.contains(id) {
                continue;
            }
            for fresh_range in &self.fresh_id_ranges {
                if fresh_range.contains(&id) {
                    total_fresh.insert(id);
                }
            }
        }
        total_fresh.len()
    }

    pub fn possible_fresh_ids(&self) -> usize {
        let x: HashSet<usize> = self
            .fresh_id_ranges
            .iter()
            .map(|it| it.clone())
            .flatten()
            .collect();
        x.len()
    }
}

pub fn parse_fridge<R: BufRead>(reader: R) -> Fridge {
    let mut lines = reader.lines();
    let mut fresh_id_ranges = Vec::new();
    // Get Fresh ID Ranges
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line == "" {
            break;
        } else {
            fresh_id_ranges.push(parse_fresh_id_range(&line))
        }
    }
    // Get IDS
    let ids = lines.map(|s| s.unwrap().parse().unwrap()).collect();

    Fridge {
        fresh_id_ranges,
        ids,
    }
}

fn parse_fresh_id_range(line: &str) -> RangeInclusive<usize> {
    let nums: Vec<usize> = line.split('-').map(|s| s.parse().unwrap()).collect();

    nums[0]..=nums[1]
}
