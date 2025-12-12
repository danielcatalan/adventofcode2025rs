use std::{io::BufRead, sync::LazyLock};
use regex::Regex;

use crate::{decoration::ChristmasDecoration, junction_box::JunctionBox};

pub fn parse_christmas_decoration<R: BufRead>(reader: R) -> ChristmasDecoration {
    let jbs = reader.lines()
        .map(|line| line.expect("Could not unwrap Line"))
        .enumerate()
        .map(|(idx,line)| parse_junctionbox(&line,idx)).collect();
    ChristmasDecoration::new(jbs)
}

fn parse_junctionbox(line: &str, circ_id:usize) -> JunctionBox{
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());
    let jb_pos: Vec<usize> =  RE.find_iter(line)
        .map(|pos|pos.as_str().parse::<usize>().unwrap())
        .collect();
    JunctionBox::new(jb_pos[0], jb_pos[1], jb_pos[2], circ_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_jb() {
        let line = "57,618,57";
        let jb = parse_junctionbox(line, 1);
        assert_eq!((57,618,57), *jb.get_postion())
        
    }
}
