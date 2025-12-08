use std::cmp::Ordering;
use Border::Start;
use Border::End;

#[derive(Debug,PartialEq, Eq)]
pub enum Border{
    Start(usize),
    End(usize)
}

impl Border{
    fn get_val(&self) -> usize{
        match self {
            Start(x) => *x,
            End(x) => *x,
        }
    }
}

impl PartialOrd for Border{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = self.cmp(other);
        Some(ord)
    }
}

impl Ord for Border{
    fn cmp(&self, other: &Self) -> Ordering {
        match (self,other) {
            (Start(val1), End(val2))=>{
                if val1 == val2{
                    return Ordering::Less;
                }
                else{
                    return val1.cmp(val2)
                }
            },
         (left,right) => {
            let l = left.get_val();
            let r = right.get_val();
            l.cmp(&r)
         }
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut borders = vec![Start(1), End(5), Start(5), End(10)];
        borders.sort();

        assert_eq!(Start(1), borders[0]);
        assert_eq!(Start(5), borders[1]);
        assert_eq!(End(5), borders[2]);
        assert_eq!(End(10), borders[3]);
    }
}