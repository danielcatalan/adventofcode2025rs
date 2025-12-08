use std::collections::{HashSet, VecDeque};
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::path::Component;

use crate::borders::Border;

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
        let mut borders: Vec<Border> = self.fresh_id_ranges
            .iter()
            .map(|r| {
                let start = Border::Start(*r.start());
                let end = Border::End(*r.end());
                let x: [Border;2] = [start,end];
                x
            })
            .flatten()
            .collect();
        borders.sort();
        
        let corrected_ranges = borders_to_corrected_ranges(&borders);

        let x = corrected_ranges
            .iter()
            .map(|r| r.end()+1 - r.start())
            .sum();
        x
    }
}

fn borders_to_corrected_ranges(borders: &Vec<Border>)-> Vec<RangeInclusive<usize>>{
    let mut corrected_ranges = Vec::new();
    let mut start_stack = VecDeque::new();
    let mut end_stack = Vec::new();
    for border in borders.iter(){
        match border {
            Border::Start(s) => start_stack.push_back(*s),
            Border::End(e) => end_stack.push(*e),
        }

        if start_stack.len() == end_stack.len(){
            let start = start_stack.pop_front().unwrap();
            let end = end_stack.pop().unwrap();
            corrected_ranges.push(start..=end);
            
            start_stack.clear();
            end_stack.clear();
        }
    }
    corrected_ranges
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
