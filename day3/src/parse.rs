pub struct Bank {
    batteries: String,
}

pub fn parse_bank(line: String) -> Bank {
    Bank { batteries: line }
}

impl Bank {
    pub fn new(s: &str) -> Self {
        Bank {
            batteries: s.to_string(),
        }
    }

    pub fn largest_jolt(&self) -> usize {
        let len_batteries = self.batteries.len();
        let batteries = self.batteries.as_bytes();
        let (idx_10s, &max_10s) = batteries[0..(len_batteries - 1)]
            .iter()
            .enumerate() //
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap(); //

        let (_idx_1s, &max_1s) = batteries[(idx_10s + 1)..]
            .iter()
            .enumerate() //
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap(); //

        let tens = (max_10s - 0x30) as usize;
        let ones = (max_1s - 0x30) as usize;
        return tens * 10 + ones;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_bank(expected: usize, s: &str) {
        let bank = Bank::new(s);
        assert_eq!(expected, bank.largest_jolt());
    }

    #[test]
    fn test_bank() {
        assert_bank(98, "987654321111111");
        assert_bank(89, "811111111111119");
        assert_bank(78, "234234234234278");
        assert_bank(92, "818181911112111");
        assert_bank(
            65,
            "1252442221213212222222222222211212142351224112221222212213421221422124234123226223512212112521243121",
        );
    }
}
