use std::io::BufRead;
use std::sync::LazyLock;
use regex::Regex;

use crate::{factory::Factory, machine::Machine};



pub fn parse_factory<R: BufRead>(reader: R) -> Factory {
    let machines = reader.lines()
        .map(|line| line.unwrap())
        .map(|line| parse_machine(&line))
        .collect();
    Factory::new(machines)
}

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d").unwrap());
 *
 */

fn parse_machine(line: &str) -> Machine{
    todo!()
}