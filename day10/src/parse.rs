use regex::Regex;
use std::io::BufRead;
use std::sync::LazyLock;

use crate::{factory::Factory, machine::Machine};

pub fn parse_factory<R: BufRead>(reader: R) -> Factory {
    let machines = reader
        .lines()
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

fn parse_machine(line: &str) -> Machine {
    let led_mask = parse_leds(line);
    let button_wirings = parse_buttons(line);
    Machine::new(led_mask, button_wirings)
}

fn parse_leds(line: &str) -> usize {
    static RE_LEDS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\[(?P<leds>[#.]{1,})\]").unwrap());
    let caps = RE_LEDS
        .captures(line)
        .expect("Expected Led mask during regex parse");
    let leds = caps["leds"].as_bytes();
    let mut led_mask = 0;
    for (i, b) in leds.iter().enumerate() {
        if *b == b'#' {
            led_mask |= 1 << i;
        }
    }
    led_mask
}

fn parse_buttons(line: &str) -> Vec<usize> {
    static RE_BUTTONS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\((?P<button>[0-9,]{1,})\)").unwrap());
    let button_wirings = RE_BUTTONS
        .captures_iter(line)
        .map(|cap| cap["button"].to_owned())
        .map(|button| parse_wiring(&button))
        .collect();
    button_wirings
}

fn parse_wiring(s: &str) -> usize {
    let wiring = s
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .map(|b| 1 << b)
        .fold(0, |a, b| a | b);
    wiring
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_leds() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let led_mask = parse_leds(line);
        assert_eq!(8, led_mask);

        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let led_mask = parse_leds(line);
        assert_eq!(6, led_mask)
    }

    #[test]
    fn test_parse_button_wiring() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let button_wiring_masks = parse_buttons(line);
        assert_eq!(6, button_wiring_masks.len());
        assert_eq!(8, button_wiring_masks[0]); // (3)
        assert_eq!(10, button_wiring_masks[1]); // (1,3)
        assert_eq!(4, button_wiring_masks[2]); // (2)
        assert_eq!(3, button_wiring_masks[5]); // (0,1)
    }
}
