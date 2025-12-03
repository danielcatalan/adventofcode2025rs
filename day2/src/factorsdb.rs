use std::collections::HashMap;

pub struct FactorsDB {
    db: HashMap<usize, Vec<usize>>,
}

impl FactorsDB {
    pub fn new() -> Self {
        let mut factor_db = FactorsDB { db: HashMap::new() };

        for i in 1..12 {
            factor_db.init_factors(i);
        }
        factor_db
    }

    pub fn factors_of(&self, val: usize) -> &Vec<usize> {
        self.db.get(&val).unwrap()
    }

    fn init_factors(&mut self, val: usize) {
        let mut factors = Vec::new();
        for i in 1..val {
            if val.is_multiple_of(i) {
                factors.push(i as usize);
            }
        }
        self.db.insert(val, factors.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factors() {
        let fact_db = FactorsDB::new();

        let factors = fact_db.factors_of(2); //eg len(22)
        assert_eq!(vec![1], *factors);
        let factors = fact_db.factors_of(3);
        assert_eq!(vec![1], *factors);
        let factors = fact_db.factors_of(4);
        assert_eq!(vec![1, 2], *factors);
        let factors = fact_db.factors_of(6);
        assert_eq!(vec![1, 2, 3], *factors);
        let factors = fact_db.factors_of(9);
        assert_eq!(vec![1, 3], *factors);
    }
}
