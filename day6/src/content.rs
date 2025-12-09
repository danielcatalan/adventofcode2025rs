use crate::content::Content::{Digit, None, Op};
use crate::operations::Operation;

#[derive(Debug, PartialEq)]
pub enum Content {
    Digit(u8),
    Op(Operation),
    None,
}

impl Content {
    pub(crate) fn new(b: u8) -> Self {
        match b {
            b'0'..=b'9' => Digit(b - 0x30),
            b'+' => Op(Operation::Add),
            b'*' => Op(Operation::Mult),
            b' ' => None,
            _ => todo!(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_new() {
        let x = Content::new(b'5');
        assert_eq!(Digit(5), x);

        let x = Content::new(b' ');
        assert_eq!(None, x);
    }
}
