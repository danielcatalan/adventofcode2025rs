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
            .rev()
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap(); //

        let (_idx_1s, &max_1s) = batteries[(idx_10s + 1)..]
            .iter()
            .enumerate() //
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap(); //

        let tens = (max_10s - 0x30) as usize;
        let ones = (max_1s - 0x30) as usize;
        return (tens * 10) + ones;
    }
    pub fn largest_jolt2(&self) -> usize {
        self.largest_jolt_impl(12)
    }

    pub fn largest_jolt_impl(&self, digits: usize) -> usize {
        let bank_len = self.batteries.len();
        let batteries: &Vec<(usize, u8)> = &self.batteries.bytes().enumerate().collect();
        let mut start_idx = 0;
        let mut jolt: usize = 0;
        for dig in 0..digits {
            let block_end = bank_len - (digits - (dig + 1));
            let (idx, bat) = batteries[start_idx..block_end]
                .iter()
                .rev()
                .max_by(|x, y| x.1.cmp(&y.1))
                .unwrap();
            jolt = jolt * 10 + (bat - 0x30) as usize;
            start_idx = idx + 1;
        }
        jolt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_bank(expected: usize, s: &str) {
        let bank = Bank::new(s);
        assert_eq!(expected, bank.largest_jolt());
    }

    fn assert_bank_part2(expected: usize, s: &str) {
        let bank = Bank::new(s);
        assert_eq!(expected, bank.largest_jolt2());
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
        assert_bank(
            99,
            "74661582914373377771857781284845741681685815142631524817317361384343713861915348743343524472515165481",
        );
    }

    #[test]
    fn test_bank_part2() {
        assert_bank_part2(987654321111, "987654321111111");
        assert_bank_part2(811111111119, "811111111111119");
        assert_bank_part2(434234234278, "234234234234278");
        assert_bank_part2(888911112111, "818181911112111");
    }
}
