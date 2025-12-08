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
        let mut corrected_ranges = Vec::new();
        for fresh_range in self.fresh_id_ranges.iter() {
            correct_range(&mut corrected_ranges, fresh_range);
        }

        // count ranges
        corrected_ranges.iter().map(|r| r.end() - r.start()).sum()
    }
}

fn correct_range(
    corrected_ranges: &mut Vec<RangeInclusive<usize>>,
    unique_range: &RangeInclusive<usize>,
) {
    for corrected_range in corrected_ranges.iter_mut() {
        if let Some(range) = is_overlap(corrected_range, unique_range) {
            *corrected_range = range;
            return;
        }
    }
    corrected_ranges.push(unique_range.clone());
}
fn is_overlap(
    corrected_range: &RangeInclusive<usize>,
    unique_range: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    if corrected_range.contains(unique_range.start())
        || corrected_range.contains(unique_range.end())
    {
        let start = if corrected_range.start() < unique_range.start() {
            corrected_range.start()
        } else {
            unique_range.start()
        };
        let end = if corrected_range.end() > unique_range.end() {
            corrected_range.end()
        } else {
            unique_range.end()
        };
        return Some(*start..=*end);
    }
    return None;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let x = 0..=3;
        assert_eq!(4, x.end() + 1 - x.start());
    }
}
